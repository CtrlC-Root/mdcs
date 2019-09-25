use mdcs::device::Device;
use sensors;

fn main() {
    let device = Device::new();
    println!("{:#?}", device);
}
