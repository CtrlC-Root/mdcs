use std::io;
use std::error::Error;
use std::result::Result;
use std::net::{TcpListener, TcpStream};

use avro_rs::{
    Schema,
    Reader,
    Writer,
    from_value
};

use mdcs::device::Device;
use super::request::{self, PluginRequest};
use super::response::{self, PluginResponse};

pub struct PluginServer {
    device: Device
}

fn initial_connect() -> io::Result<TcpStream> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    println!("LISTENING {}:{}", address.ip(), address.port());

    match listener.accept() {
        Ok((stream, address)) => {
            println!("ACCEPTED {}:{}", address.ip(), address.port());
            Ok(stream)
        }
        Err(error) => Err(error)
    }
}

impl PluginServer {
    pub fn new(device: Device) -> PluginServer {
        PluginServer {
            device
        }
    }

    fn process_request(&mut self, request: &PluginRequest) -> PluginResponse {
        // TODO: implement me
        PluginResponse::Status(response::Status::Ok)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // parse the plugin message schemas
        let request_schema = Schema::parse_str(include_str!("request.avsc"))
            .expect("Failed to parse request message schema");

        let response_schema = Schema::parse_str(include_str!("response.avsc"))
            .expect("Failed to parse response message schema");

        // wait for someone to connect
        let stream = initial_connect()?;

        // create plugin request reader and response writer
        let reader = Reader::with_schema(&request_schema, &stream)?;
        let mut writer = Writer::new(&response_schema, &stream);

        // process requests
        for value in reader {
            let value = value.expect("Failed to read request");
            let request = from_value::<PluginRequest>(&value);

            let response = match request {
                Ok(request) => {
                    self.process_request(&request)
                },
                Err(error) => PluginResponse::Error(response::Error {
                    message: String::from(format!("{}", error)),
                    path: None
                })
            };

            writer.append_ser(response).expect("Failed to serialize response");
            writer.flush().expect("Failed to flush Avro writer");
        }

        // quit
        Ok(())
    }
}
