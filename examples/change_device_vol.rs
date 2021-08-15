use std::io::{self, Write};

use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;

fn main() {
    // create handler that calls functions on playback devices and apps
    let mut handler = SinkController::create().unwrap();
    let devices = handler
        .list_devices()
        .expect("Could not get list of playback devices");

    println!("Playback Devices: ");
    for device in &devices {
        println!(
            "[Index: {}] {}, [Volume: {}]",
            device.index,
            device.description.as_ref().unwrap(),
            device.volume
        );
    }

    let mut input = String::new();

    print!("Select an index: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    println!("Increasing volume by 5%...");

    let device_index = input.trim().parse().expect("Invalid number");
    handler.increase_device_volume_by_percent(device_index, 0.05);

    println!(
        "Volume set to [Volume: {}]",
        handler
            .get_device_by_index(device_index)
            .expect("Failed to get device by index")
            .volume
    )
}
