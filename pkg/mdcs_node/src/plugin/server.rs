use std::error::Error;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::result::Result;

use avro_rs::{from_avro_datum, from_value, to_avro_datum, Reader, Schema, Writer};

use mdcs::avro;
use mdcs::device::{AttributeFlags, Device, Member};

use super::request::{self as req, Request};
use super::response::{self as resp, Response};

pub struct Server {
    device: Device,
    signal_quit: bool,
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
        Err(error) => Err(error),
    }
}

impl Server {
    pub fn new(device: Device) -> Server {
        Server {
            device,
            signal_quit: false,
        }
    }

    fn signal(&mut self, signal: &req::Signal) -> Response {
        match signal {
            req::Signal::Quit => {
                self.signal_quit = true;
                Response::Status(resp::Status::Ok)
            }
        }
    }

    fn describe_device(&mut self) -> Response {
        let mut attributes: Vec<resp::Attribute> = vec![];
        let mut actions: Vec<resp::Action> = vec![];

        for member in self.device.iter() {
            match member {
                (path, Member::Attribute(attribute)) => {
                    let mut flags: Vec<String> = vec![];
                    for flag in attribute.flags() {
                        flags.push(match flag {
                            AttributeFlags::Read => "read".to_string(),
                            AttributeFlags::Write => "write".to_string(),
                        });
                    }

                    let schema = match serde_json::to_string(&attribute.schema()) {
                        Ok(schema) => schema,
                        Err(error) => {
                            return Response::Error(resp::Error {
                                message: format!("Failed to serialize attribute schema: {}", error),
                                path: Some(path.clone()),
                            });
                        }
                    };

                    attributes.push(resp::Attribute {
                        path: path.clone(),
                        flags,
                        schema,
                    });
                }
                (path, Member::Action(action)) => {
                    let input_schema = match serde_json::to_string(&action.input_schema()) {
                        Ok(schema) => schema,
                        Err(error) => {
                            return Response::Error(resp::Error {
                                message: format!(
                                    "Failed to serialize action input schema: {}",
                                    error
                                ),
                                path: Some(path.clone()),
                            });
                        }
                    };

                    let output_schema = match serde_json::to_string(&action.output_schema()) {
                        Ok(schema) => schema,
                        Err(error) => {
                            return Response::Error(resp::Error {
                                message: format!(
                                    "Failed to serialize action output schema: {}",
                                    error
                                ),
                                path: Some(path.clone()),
                            });
                        }
                    };

                    actions.push(resp::Action {
                        path: path.clone(),
                        input_schema,
                        output_schema,
                    })
                }
            }
        }

        Response::Device(resp::Device {
            attributes,
            actions,
        })
    }

    fn read_attribute(&mut self, args: &req::ReadAttribute) -> Response {
        // retrieve the device attribute
        let attribute = match self.device.get(&args.path) {
            Some(Member::Attribute(attribute)) => attribute,
            Some(_) => {
                return Response::Error(resp::Error {
                    message: "Path does not refer to an attribute".to_string(),
                    path: Some(args.path.clone()),
                });
            }
            None => {
                return Response::Error(resp::Error {
                    message: "Path not found".to_string(),
                    path: Some(args.path.clone()),
                });
            }
        };

        // verify the attribute is readable
        if !attribute.readable() {
            return Response::Error(resp::Error {
                message: "Attribute not readable".to_string(),
                path: Some(args.path.clone()),
            });
        }

        // retrieve the current system time
        let time = avro::timestamp();

        // read the attribute value
        let value = match attribute.read() {
            Ok(value) => value,
            Err(error) => {
                return Response::Error(resp::Error {
                    message: format!("Failed to read attribute: {}", error),
                    path: Some(args.path.clone()),
                });
            }
        };

        // encode the attribute value
        let schema = attribute.schema();
        let value = match to_avro_datum(&schema, value) {
            Ok(bytes) => bytes,
            Err(error) => {
                return Response::Error(resp::Error {
                    message: format!("Failed to serialize value: {}", error),
                    path: Some(args.path.clone()),
                });
            }
        };

        Response::AttributeValue(resp::AttributeValue { value, time })
    }

    fn write_attribute(&mut self, args: &req::WriteAttribute) -> Response {
        // retrieve the device attribute
        let attribute = match self.device.get(&args.path) {
            Some(Member::Attribute(attribute)) => attribute,
            Some(_) => {
                return Response::Error(resp::Error {
                    message: "Path does not refer to an attribute".to_string(),
                    path: Some(args.path.clone()),
                });
            }
            None => {
                return Response::Error(resp::Error {
                    message: "Path not found".to_string(),
                    path: Some(args.path.clone()),
                });
            }
        };

        // verify the attribute is writable
        if !attribute.writable() {
            return Response::Error(resp::Error {
                message: "Attribute not writable".to_string(),
                path: Some(args.path.clone()),
            });
        }

        // decode the attribute value
        let schema = attribute.schema();
        let value = args.value.clone();
        let mut buffer: &[u8] = &value[..];

        let decoded_value = match from_avro_datum(&schema, &mut buffer, None) {
            Ok(value) => value,
            Err(error) => {
                return Response::Error(resp::Error {
                    message: format!("Failed to unserialize value: {}", error),
                    path: Some(args.path.clone()),
                });
            }
        };

        // retrieve the current system time
        let time = avro::timestamp();

        // write the attribute value
        if let Err(error) = attribute.write(decoded_value) {
            return Response::Error(resp::Error {
                message: format!("Failed to write attribute: {}", error),
                path: Some(args.path.clone()),
            });
        };

        Response::AttributeValue(resp::AttributeValue { value, time })
    }

    fn run_action(&mut self, args: &req::RunAction) -> Response {
        // retrieve the device action
        let action = match self.device.get(&args.path) {
            Some(Member::Action(action)) => action,
            Some(_) => {
                return Response::Error(resp::Error {
                    message: "Path does not refer to an action".to_string(),
                    path: Some(args.path.clone()),
                });
            }
            None => {
                return Response::Error(resp::Error {
                    message: "Path not found".to_string(),
                    path: Some(args.path.clone()),
                });
            }
        };

        // decode the input value
        let input_schema = action.input_schema();
        let input = args.input.clone();
        let mut buffer: &[u8] = &input[..];

        let input_value = match from_avro_datum(&input_schema, &mut buffer, None) {
            Ok(value) => value,
            Err(error) => {
                return Response::Error(resp::Error {
                    message: format!("Failed to unserialize input value: {}", error),
                    path: Some(args.path.clone()),
                });
            }
        };

        // record start time
        let start = avro::timestamp();

        // run the action
        let output_value = match action.run(input_value) {
            Ok(output) => output,
            Err(error) => {
                return Response::Error(resp::Error {
                    message: format!("Failed to run action: {}", error),
                    path: Some(args.path.clone()),
                });
            }
        };

        // record end time
        let end = avro::timestamp();

        // encode the output value
        let output_schema = action.output_schema();
        let output = match to_avro_datum(&output_schema, output_value) {
            Ok(bytes) => bytes,
            Err(error) => {
                return Response::Error(resp::Error {
                    message: format!("Failed to serialize output value: {}", error),
                    path: Some(args.path.clone()),
                });
            }
        };

        Response::ActionResult(resp::ActionResult { output, start, end })
    }

    fn process_request(&mut self, request: &Request) -> Response {
        match request {
            Request::Signal(signal) => self.signal(signal),
            Request::DescribeDevice(_) => self.describe_device(),
            Request::ReadAttribute(args) => self.read_attribute(args),
            Request::WriteAttribute(args) => self.write_attribute(args),
            Request::RunAction(args) => self.run_action(args),
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
            let request = from_value::<Request>(&value);

            // process the request into a response
            let response = match request {
                Ok(request) => self.process_request(&request),
                Err(error) => Response::Error(resp::Error {
                    message: String::from(format!("{}", error)),
                    path: None,
                }),
            };

            // send the response
            writer
                .append_ser(response)
                .expect("Failed to serialize response");

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
