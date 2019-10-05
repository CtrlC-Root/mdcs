use std::io::{BufRead, BufReader, Result};
use std::net::{IpAddr, TcpStream};
use std::process::{Child, Command, Stdio};

use avro_rs::{from_avro_datum, from_value, to_avro_datum, Reader, Schema, Writer};

use super::config::InstanceConfig;
use super::request::{self as req, Request};
use super::response::{self as resp, Response};

pub struct Instance {
    config: InstanceConfig,
    child: Child,
    stream: TcpStream,
}

fn initial_connect(child: &mut Child) -> Result<TcpStream> {
    let child_stdout = child
        .stdout
        .as_mut()
        .expect("Failed to retrieve child stdout");

    let mut reader = BufReader::new(child_stdout);
    let mut listen_line = String::new();

    let length = reader.read_line(&mut listen_line)?;
    if length == 0 {
        // XXX: return an error instead
        panic!("Plugin child stdout closed");
    }

    if !listen_line.starts_with("LISTENING ") {
        // XXX: return an error instead
        panic!("Plugin output is not LISTENING line");
    }

    // XXX: return an error instead
    let address = listen_line
        .split(' ')
        .nth(1)
        .expect("Failed to parse LISTENING line");

    TcpStream::connect(address)
}

impl Instance {
    pub fn new(config: InstanceConfig) -> Result<Instance> {
        let mut child = Command::new(&config.command)
            .stdout(Stdio::piped())
            .spawn()?;

        let stream = initial_connect(&mut child)?;
        Ok(Instance {
            config,
            child,
            stream,
        })
    }

    fn send_request(&mut self, request: &Request) -> Response {
        // parse the plugin message schemas
        let request_schema = Schema::parse_str(include_str!("request.avsc"))
            .expect("Failed to parse request message schema");

        let response_schema = Schema::parse_str(include_str!("response.avsc"))
            .expect("Failed to parse response message schema");

        // create request writer and response reader
        let mut writer = Writer::new(&request_schema, &self.stream);
        let mut reader = Reader::with_schema(&response_schema, &self.stream)
            .expect("Failed to create Avro response reader");

        // send the request
        writer
            .append_ser(request)
            .expect("Failed to serialize response");

        writer.flush().expect("Failed to flush Avro writer");

        // read the response
        let value = reader.nth(0).expect("Failed to read response").expect("WTF");
        from_value::<Response>(&value).expect("Failed to unserialize response")
    }
}
