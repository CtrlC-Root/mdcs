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
    fn name(&self) -> &str {
        &self.name
    }

    fn members(&self) -> HashMap<&str, device::Member> {
        self.members
            .iter()
            .map(|(ref path, ref member)| {
                match member {
                    Member::Attribute(attribute) => (&path[..], device::Member::Attribute(attribute)),
                    Member::Action(action) => (&path[..], device::Member::Action(action))
                }
            })
            .collect()
    }
}
