use std::process::Command;
use std::result::Result;

use avro_rs::from_value;
use avro_rs::schema::Schema;
use avro_rs::types::Value;
use serde::Deserialize;

use mdcs::device::{Action, Attribute, Device, DeviceError, Member};

#[derive(Debug)]
struct IORegAttribute {
    class: String,
    property: String,
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
            .output()?;

        let stdout =
            String::from_utf8(output.stdout).map_err(|_e| "ioreg output not UTF-8 compatible")?;

        let line = stdout
            .lines()
            .filter(|line| line.contains(&self.property))
            .nth(0)
            .ok_or("ioreg output did not contain property name")?;

        let wrap_chars: &[_] = &['"', '<', '>'];
        let value = line
            .split(" = ")
            .nth(1)
            .ok_or("ioreg output property line not in expected format")?
            .trim_matches(wrap_chars);

        Ok(Value::String(value.to_string()))
    }
}

#[derive(Debug)]
struct SayAction {}

#[derive(Debug, Deserialize)]
struct SaySettings {
    msg: String,
    voice: Option<String>,
    rate: Option<i32>,
}

impl Action for SayAction {
    fn input_schema(&self) -> Schema {
        let raw_schema = r#"
            {
                "type": "record",
                "name": "SaySettings",
                "fields": [
                    {"name": "msg", "type": "string"},
                    {"name": "voice", "type": ["null", "string"]},
                    {"name": "rate", "type": ["null", "int"]},
                ]
            }
        "#;

        Schema::parse_str(raw_schema).expect("Failed to parse embedded schema")
    }

    fn output_schema(&self) -> Schema {
        Schema::Null
    }

    fn run(&self, input: Value) -> Result<Value, DeviceError> {
        let settings =
            from_value::<SaySettings>(&input).expect("Action input does not match schema");

        let mut args: Vec<String> = vec![];

        if let Some(voice) = settings.voice {
            args.push("-v".to_string());
            args.push(voice.clone());
        }

        if let Some(rate) = settings.rate {
            args.push("-r".to_string());
            args.push(rate.to_string());
        }

        args.push(settings.msg.clone());
        let output = Command::new("/usr/bin/say").args(args).output()?;

        if !output.status.success() {
            let stderr = String::from_utf8(output.stdout)
                .map_err(|_e| "ioreg output not UTF-8 compatible")?;

            return Err(From::from(format!("say failed: {}", stderr.trim())));
        }

        Ok(Value::Null)
    }
}

pub fn platform_attributes(device: &mut Device) {
    // serial number
    let attribute = Box::new(IORegAttribute {
        class: "IOPlatformExpertDevice".to_string(),
        property: "IOPlatformSerialNumber".to_string(),
    });

    device
        .insert("serial", Member::Attribute(attribute))
        .unwrap();

    // model number
    let attribute = Box::new(IORegAttribute {
        class: "IOPlatformExpertDevice".to_string(),
        property: "model".to_string(),
    });

    device
        .insert("model", Member::Attribute(attribute))
        .unwrap();

    // say
    let action = Box::new(SayAction {});
    device.insert("say", Member::Action(action)).unwrap();
}
