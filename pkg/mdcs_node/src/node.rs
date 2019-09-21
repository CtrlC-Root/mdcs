use std::collections::HashMap;
use mdcs::device::Device;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub devices: HashMap<String, Device>
}
