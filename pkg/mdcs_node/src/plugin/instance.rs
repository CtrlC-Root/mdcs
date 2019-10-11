use std::io::{self, BufRead, BufReader, Result, ErrorKind};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};

use avro_rs::{from_avro_datum, from_value, to_avro_datum, Reader, Schema, Writer};

use super::config::InstanceConfig;
use super::request::{self as req, Request};
use super::response::{self as resp, Response};


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

pub struct Instance {
    config: InstanceConfig,
    child: Child,
    stream: TcpStream,
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
}
