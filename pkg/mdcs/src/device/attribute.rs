use std::fmt;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum AttributeFlags {
    Read,
    Write
}

pub trait Attribute {
    fn schema(&self) -> &str;

    fn readable(&self) -> bool;
    fn writable(&self) -> bool;

    fn flags(&self) -> HashSet<AttributeFlags> {
        let mut flags = HashSet::new();

        if self.readable() {
            flags.insert(AttributeFlags::Read);
        }

        if self.writable() {
            flags.insert(AttributeFlags::Write);
        }

        return flags;
    }
}

impl fmt::Debug for Box<dyn Attribute> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Attribute")
            .field("schema", &self.schema())
            .field("flags", &self.flags())
            .finish()
    }
}
