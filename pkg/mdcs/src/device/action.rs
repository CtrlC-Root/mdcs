use std::fmt;

use avro_rs::schema::Schema;
use avro_rs::types::Value;

use super::error::{DeviceError, ErrorKind};

pub trait Action {
    fn input_schema(&self) -> Schema;
    fn output_schema(&self) -> Schema;

    fn run(&self, _input: Value) -> Result<Value, DeviceError> {
        Err(DeviceError::new(ErrorKind::Generic))
    }
}

impl fmt::Debug for Box<dyn Action> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Action")
            .field("input_schema", &self.input_schema())
            .field("output_schema", &self.output_schema())
            .finish()
    }
}
