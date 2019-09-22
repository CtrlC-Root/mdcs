use std::fmt;
use std::collections::HashMap;

pub trait Attribute {
    fn schema(&self) -> &str;
    // fn read(&self);
    // fn write(&self);
}

impl fmt::Debug for &dyn Attribute {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Attribute")
            .field("schema", &self.schema())
            .finish()
    }
}

pub trait Action {
    fn input_schema(&self) -> &str;
    fn output_schema(&self) -> &str;
    // fn run(&self);
}

impl fmt::Debug for &dyn Action {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Action")
            .field("input_schema", &self.input_schema())
            .field("output_schema", &self.output_schema())
            .finish()
    }
}

pub trait Device {
    fn name(&self) -> &str;
    fn attributes(&self) -> &HashMap<&str, &dyn Attribute>;
    fn actions(&self) -> &HashMap<&str, &dyn Action>;
}

impl fmt::Debug for &dyn Device {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Device")
            .field("name", &self.name())
            .field("attributes", &self.attributes())
            .field("actions", &self.actions())
            .finish()
    }
}
