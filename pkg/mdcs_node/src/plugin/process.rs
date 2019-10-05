use std::io;
use std::error::Error;
use std::result::Result;
use std::net::{TcpListener, TcpStream};

use avro_rs::{
    Schema,
    Reader,
    Writer,
    from_value,
    to_avro_datum,
    from_avro_datum
};

use mdcs::device::{
    Member,
    Device
};
use mdcs::avro;
use super::request::{
    self as req,
    PluginRequest
};
use super::response::{
    self as resp,
    PluginResponse
};

pub struct PluginServer {
    device: Device,
    signal_quit: bool
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
            device,
            signal_quit: false
        }
    }

    fn signal(&mut self, signal: &req::Signal) -> PluginResponse {
        match signal {
            req::Signal::Quit => {
                self.signal_quit = true;
                PluginResponse::Status(resp::Status::Ok)
            }
        }
    }

    fn describe_device(&mut self) -> PluginResponse {
        // TODO: implement this
        PluginResponse::Error(resp::Error {
            message: String::from("Not Implemented"),
            path: None
        })
    }

    fn read_attribute(&mut self, args: &req::ReadAttribute) -> PluginResponse {
        // retrieve the device attribute
        let attribute = match self.device.get(&args.path) {
            Some(Member::Attribute(attribute)) => attribute,
            Some(Member::Action(_)) => {
                return PluginResponse::Error(resp::Error {
                    message: "Path does not refer to an attribute".to_string(),
                    path: Some(args.path.clone())
                });
            }
            None => {
                return PluginResponse::Error(resp::Error {
                    message: "Path not found".to_string(),
                    path: Some(args.path.clone())
                });
            }
        };

        // verify the attribute is readable
        if !attribute.readable() {
            return PluginResponse::Error(resp::Error {
                message: "Attribute not readable".to_string(),
                path: Some(args.path.clone())
            });
        }

        // retrieve the current system time
        let timestamp = avro::timestamp();

        // read the attribute value
        let value = match attribute.read() {
            Ok(value) => value,
            Err(error) => {
                return PluginResponse::Error(resp::Error {
                    message: format!("Failed to read attribute: {}", error),
                    path: Some(args.path.clone())
                });
            }
        };

        // encode the attribute value
        let schema = attribute.schema();
        let encoded_value = match to_avro_datum(&schema, value) {
            Ok(bytes) => bytes,
            Err(error) => {
                return PluginResponse::Error(resp::Error {
                    message: format!("Failed to serialize value: {}", error),
                    path: Some(args.path.clone())
                });
            }
        };

        PluginResponse::AttributeValue(resp::AttributeValue {
            value: encoded_value,
            time: timestamp
        })
    }

    fn write_attribute(&mut self, args: &req::WriteAttribute) -> PluginResponse {
        // retrieve the device attribute
        let attribute = match self.device.get(&args.path) {
            Some(Member::Attribute(attribute)) => attribute,
            Some(Member::Action(_)) => {
                return PluginResponse::Error(resp::Error {
                    message: "Path does not refer to an attribute".to_string(),
                    path: Some(args.path.clone())
                });
            }
            None => {
                return PluginResponse::Error(resp::Error {
                    message: "Path not found".to_string(),
                    path: Some(args.path.clone())
                });
            }
        };

        // verify the attribute is writable
        if !attribute.writable() {
            return PluginResponse::Error(resp::Error {
                message: "Attribute not writable".to_string(),
                path: Some(args.path.clone())
            });
        }

        // decode the attribute value
        let schema = attribute.schema();
        let value = args.value.clone();
        let mut buffer: &[u8] = &value[..];

        let decoded_value = match from_avro_datum(&schema, &mut buffer, None) {
            Ok(value) => value,
            Err(error) => {
                return PluginResponse::Error(resp::Error {
                    message: format!("Failed to unserialize value: {}", error),
                    path: Some(args.path.clone())
                });
            }
        };

        // retrieve the current system time
        let timestamp = avro::timestamp();

        // write the attribute value
        if let Err(error) = attribute.write(decoded_value) {
            return PluginResponse::Error(resp::Error {
                message: format!("Failed to write attribute: {}", error),
                path: Some(args.path.clone())
            });
        };

        PluginResponse::AttributeValue(resp::AttributeValue {
            value: value,
            time: timestamp
        })
    }

    fn run_action(&mut self, args: &req::RunAction) -> PluginResponse {
        // TODO: implement this
        PluginResponse::Error(resp::Error {
            message: String::from("Not Implemented"),
            path: Some(args.path.clone())
        })
    }

    fn process_request(&mut self, request: &PluginRequest) -> PluginResponse {
        match request {
            PluginRequest::Signal(signal) => self.signal(signal),
            PluginRequest::DescribeDevice(_) => self.describe_device(),
            PluginRequest::ReadAttribute(args) => self.read_attribute(args),
            PluginRequest::WriteAttribute(args) => self.write_attribute(args),
            PluginRequest::RunAction(args) => self.run_action(args),
        }
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
            // parse the request
            let value = value.expect("Failed to read request");
            let request = from_value::<PluginRequest>(&value);

            // process the request into a response
            let response = match request {
                Ok(request) => {
                    self.process_request(&request)
                },
                Err(error) => PluginResponse::Error(resp::Error {
                    message: String::from(format!("{}", error)),
                    path: None
                })
            };

            // send the response
            writer.append_ser(response).expect("Failed to serialize response");
            writer.flush().expect("Failed to flush Avro writer");

            // quit if necessary
            if self.signal_quit {
                println!("QUIT");
                break;
            }
        }

        // finished
        Ok(())
    }
}
