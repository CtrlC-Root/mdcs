use std::fmt;

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
