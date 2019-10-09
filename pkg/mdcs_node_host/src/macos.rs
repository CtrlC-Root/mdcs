use std::result::Result;

use avro_rs::schema::Schema;
use avro_rs::types::Value;

use mdcs::device::{Attribute, Device, DeviceError, ErrorKind, Member};

pub fn make_device() -> Device {
    let mut device = Device::new();

    // TODO: something useful

    device
}
