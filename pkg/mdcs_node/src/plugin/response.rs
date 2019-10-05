use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Ok
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub path: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub path: String,
    pub flags: Vec<String>,
    pub schema: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub path: String,
    pub input_schema: String,
    pub output_schema: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub actions: Vec<Action>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeValue {
    pub value: Vec<u8>,
    pub time: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionResult {
    pub output: Vec<u8>,
    pub start: i64,
    pub end: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PluginResponse {
    Status(Status),
    Error(Error),
    Device(Device),
    AttributeValue(AttributeValue),
    ActionResult(ActionResult)
}
