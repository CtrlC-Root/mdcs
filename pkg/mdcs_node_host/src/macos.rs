use std::result::Result;
use std::process::Command;

use avro_rs::schema::Schema;
use avro_rs::types::Value;

use mdcs::device::{
    Attribute,
    Device,
    DeviceError,
    ErrorKind,
    Member
};

#[derive(Debug)]
struct IORegAttribute {
    class: String,
    property: String
}

impl Attribute for IORegAttribute {
    fn schema(&self) -> Schema {
        Schema::String
    }

    fn readable(&self) -> bool {
        true
    }

    fn read(&self) -> Result<Value, DeviceError> {
        let output = Command::new("/usr/sbin/ioreg")
            .args(&["-r", "-c", &self.class, "-k", &self.property, "-d", "1"])
            .output()
            .map_err(|e| DeviceError::from(Box::new(e)))?;

        let stdout = String::from_utf8(output.stdout)
            .map_err(|e| DeviceError::from(Box::new(e)))?;

        let line = stdout
            .lines()
            .filter(|line| line.contains(&self.property))
            .nth(0)
            .ok_or(DeviceError::new(ErrorKind::Generic))?;

        let wrap_chars: &[_] = &['"', '<', '>'];
        let value = line
            .split(" = ")
            .nth(1)
            .ok_or(DeviceError::new(ErrorKind::Generic))?
            .trim_matches(wrap_chars);

        Ok(Value::String(value.to_string()))
    }
}

pub fn make_device() -> Device {
    let mut device = Device::new();

    // serial number
    let attribute = Box::new(IORegAttribute {
        class: "IOPlatformExpertDevice".to_string(),
        property: "IOPlatformSerialNumber".to_string()
    });

    device.insert("serial", Member::Attribute(attribute)).unwrap();

    // model number
    let attribute = Box::new(IORegAttribute {
        class: "IOPlatformExpertDevice".to_string(),
        property: "model".to_string()
    });

    device.insert("model", Member::Attribute(attribute)).unwrap();

    device
}
