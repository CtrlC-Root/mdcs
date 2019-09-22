use std::collections::HashMap;
use mdcs::device::Device;

#[derive(Debug)]
pub struct Node<'a> {
    pub name: String,
    pub devices: HashMap<String, &'a dyn Device>
}
