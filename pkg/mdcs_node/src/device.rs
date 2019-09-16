use std::collections::HashMap;

#[derive(Debug)]
pub struct Attribute {
    pub schema: String
}

#[derive(Debug)]
pub struct Function {
    pub input_schema: String,
    pub output_schema: String
}

#[derive(Debug)]
pub enum Member {
    Attribute(Attribute),
    Function(Function)
}

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub members: HashMap<String, Member>
}
