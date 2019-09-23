use std::collections::HashMap;
use mdcs::device;

#[derive(Debug)]
pub struct Attribute {
    pub schema: String
}

impl device::Attribute for Attribute {
    fn schema(&self) -> &str {
        &self.schema
    }

    fn readable(&self) -> bool {
        return true;
    }

    fn writable(&self) -> bool {
        return false;
    }
}

#[derive(Debug)]
pub struct Action {
    pub input_schema: String,
    pub output_schema: String
}

impl device::Action for Action {
    fn input_schema(&self) -> &str {
        &self.input_schema
    }

    fn output_schema(&self) -> &str {
        &self.output_schema
    }
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

impl device::Device for Device {
    // TODO
}
