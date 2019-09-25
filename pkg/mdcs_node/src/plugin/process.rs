use std::io;
use std::error::Error;
use std::result::Result;
use std::net::{TcpListener, TcpStream};

use mdcs::device::Device;

pub struct PluginServer {
    device: Device
}

fn initial_connect() -> io::Result<TcpStream> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    println!("LISTENING {}:{}", address.ip(), address.port());

    loop {
        match listener.accept() {
            Ok((stream, address)) => {
                println!("ACCEPTED {}:{}", address.ip(), address.port());
                return Ok(stream);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
}

impl PluginServer {
    pub fn new(device: Device) -> PluginServer {
        PluginServer {
            device
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let stream = initial_connect()?;
        stream.set_nonblocking(true)?;

        // TODO: process requests
        Ok(())
    }
}
