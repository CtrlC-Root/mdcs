use std::collections::HashMap;
use crate::device::Device;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub devices: HashMap<String, Device>
}
