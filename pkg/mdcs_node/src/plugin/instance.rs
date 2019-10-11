use std::io::{self, BufRead, BufReader, Result, ErrorKind};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use avro_rs::{from_value, Reader, Schema, Writer};

use super::config::InstanceConfig;
use super::request::Request;
use super::response::Response;

fn initial_connect(child: &mut Child) -> Result<TcpStream> {
    let child_stdout = child
        .stdout
        .as_mut()
        .expect("Failed to retrieve child stdout");

    let mut reader = BufReader::new(child_stdout);
    let mut listen_line = String::new();

    let length = reader.read_line(&mut listen_line)?;
    if length == 0 {
        return Err(
            io::Error::new(ErrorKind::BrokenPipe, "read zero length string from child stdout")
        );
    }

    if !listen_line.starts_with("LISTENING ") {
        return Err(
            io::Error::new(ErrorKind::InvalidData, "expected LISTENING line from child stdout")
        );
    }

    let address = listen_line
        .split(' ')
        .nth(1)
        .ok_or(io::Error::new(ErrorKind::InvalidData, "received invalid LISTENING line"))?;

    TcpStream::connect(address)
}

#[derive(Debug)]
pub struct Instance {
    config: InstanceConfig,
    child: Child,
    handle: JoinHandle<()>,
    sender: Sender<Arc<InstanceMessage>>,
}

#[derive(Debug)]
struct InstanceMessage {
    request: Request,
    response: Option<Response>
}

#[derive(Debug)]
struct InstanceContext {
    stream: TcpStream,
    receiver: Receiver<Arc<InstanceMessage>>,
}

impl Instance {
    pub fn new(config: InstanceConfig) -> Result<Instance> {
        let mut child = Command::new(&config.command)
            .stdout(Stdio::piped())
            .spawn()?;

        let stream = initial_connect(&mut child)?;
        let (sender, receiver) = channel();
        let mut context = InstanceContext {
            stream,
            receiver
        };

        let handle = thread::spawn(move || {
            context.run();
        });

        Ok(Instance {
            config,
            child,
            handle,
            sender,
        })
    }

    pub fn process_request(&mut self, request: Request) -> Response {
        let message = Arc::new(InstanceMessage {
            request,
            response: None
        });

        self.sender.send(message.clone()).expect("Failed to send message");

        // TODO: wait for command to finish or time out
        thread::sleep(Duration::from_millis(1000 * 5));

        let message = Arc::try_unwrap(message).expect("Existing references to Arc");
        message.response.expect("Failed to process request")
    }
}

impl InstanceContext {
    fn run(&mut self) {
        // parse the plugin message schemas
        let request_schema = Schema::parse_str(include_str!("request.avsc"))
            .expect("Failed to parse request message schema");

        let response_schema = Schema::parse_str(include_str!("response.avsc"))
            .expect("Failed to parse response message schema");

        // create request writer and response reader
        let mut writer = Writer::new(&request_schema, &self.stream);
        let mut reader = Reader::with_schema(&response_schema, &self.stream)
            .expect("Failed to create Avro response reader");

        // process requests
        loop {
            let mut message = match self.receiver.try_recv() {
                Ok(message) => message,
                Err(TryRecvError::Empty) => {
                    // rate limit
                    thread::sleep(Duration::from_millis(100));

                    // try again
                    continue;
                },
                Err(TryRecvError::Disconnected) => {
                    // time to quit
                    break;
                }
            };

            // XXX: not sure if this is the best way
            let message = Arc::get_mut(&mut message)
                .expect("Failed to get mutable message reference");

            // send the request
            writer.append_ser(&message.request).expect("Failed to serialize request");
            writer.flush().expect("Failed to flush writer");

            // receive the response
            let value = reader
                .next()
                .expect("Failed to read response")
                .expect("Failed to unserialize response");

            message.response = Some(
                from_value::<Response>(&value).expect("Failed to unserialize response")
            );

            // TODO: notify whoever is waiting the command is complete
        }
    }
}
