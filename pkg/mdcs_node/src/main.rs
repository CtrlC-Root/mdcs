use std::collections::HashMap;
use mdcs::device;
use mdcs_node::node::{Attribute, Action, Member, Device};

fn main() {
    let serial = Attribute {
        schema: String::from("string")
    };

    let beep = Action {
        input_schema: String::from("int"),
        output_schema: String::from("int")
    };

    let mut light = Device {
        name: String::from("light"),
        members: HashMap::new()
    };

    light.members.insert(String::from("serial"), Member::Attribute(serial));
    light.members.insert(String::from("beep"), Member::Action(beep));

    println!("{:#?}\n", light);

    let device: &dyn device::Device = &light;
    println!("{:#?}", device);
}
