use std::collections::HashMap;

#[derive(Debug)]
pub struct Attribute {
    pub schema: String
}

#[derive(Debug)]
pub struct Action {
    pub input_schema: String,
    pub output_schema: String
}

#[derive(Debug)]
pub enum Member {
    Attribute(Attribute),
    Action(Action)
}

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub members: HashMap<String, Member>
}
