use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Signal {
    Quit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DescribeDevice {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadAttribute {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteAttribute {
    pub path: String,
    pub value: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunAction {
    pub path: String,
    pub input: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Signal(Signal),
    DescribeDevice(DescribeDevice),
    ReadAttribute(ReadAttribute),
    WriteAttribute(WriteAttribute),
    RunAction(RunAction),
}
