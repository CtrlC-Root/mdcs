use std::fmt;

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
