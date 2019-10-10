use std::result::Result;

use avro_rs::schema::Schema;
use avro_rs::types::Value;
use sensors::{FeatureType, Sensors, SubfeatureType};

use mdcs::device::{Attribute, Device, DeviceError, ErrorKind, Member};

struct TempAttribute {
    chip_address: i32,
    feature_number: i32,
}

impl Attribute for TempAttribute {
    fn schema(&self) -> Schema {
        Schema::Double
    }

    fn readable(&self) -> bool {
        true
    }

    fn read(&self) -> Result<Value, DeviceError> {
        let sensors = Sensors::new();
        let chip = sensors
            .into_iter()
            .find(|c| c.address() == self.chip_address)
            .ok_or(DeviceError::new(ErrorKind::OutOfSync))?;

        let feature = chip
            .into_iter()
            .find(|f| f.number() == self.feature_number)
            .ok_or(DeviceError::new(ErrorKind::OutOfSync))?;

        let subfeature = feature
            .get_subfeature(SubfeatureType::SENSORS_SUBFEATURE_TEMP_INPUT)
            .ok_or(DeviceError::new(ErrorKind::OutOfSync))?;

        match subfeature.get_value() {
            Ok(value) => Ok(Value::Double(value)),
            Err(error) => Err(DeviceError::from(Box::new(error))),
        }
    }
}

pub fn platform_attributes(device: &mut Device) {
    let sensors = Sensors::new();

    for chip in sensors {
        let chip_name = chip.get_name().unwrap();
        let chip_address = chip.address();

        for feature in chip {
            let feature_name = feature.name().to_string();
            let feature_number = feature.number();

            let mut attribute: Option<Box<dyn Attribute>> = match feature.feature_type() {
                FeatureType::SENSORS_FEATURE_TEMP => Some(Box::new(TempAttribute {
                    chip_address,
                    feature_number,
                })),
                _ => None,
            };

            if let Some(attribute) = attribute.take() {
                let path = format!("{}.{}", &chip_name, &feature_name);
                device.insert(&path, Member::Attribute(attribute)).unwrap();
            }
        }
    }
}
