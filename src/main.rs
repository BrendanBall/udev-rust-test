extern crate udev;

fn main() {
    let context = udev::Context::new().unwrap();
    let mut enumerator = udev::Enumerator::new(&context).unwrap();

    // enumerator.match_subsystem("misc").unwrap();
	// enumerator.match_sysname("uinput").unwrap();

  for device in enumerator.scan_devices().unwrap() {
    println!("found device: {:?}", device.syspath());
  }
}