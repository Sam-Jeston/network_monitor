extern crate argparse;
extern crate pcap;

use argparse::{ArgumentParser, Store, StoreTrue};
use pcap::{Capture, Device, Stat};

// Print available devices
fn print_available_devices<'a> (vec_devices : &'a Vec<Device>) {
    println!("Available Devices: ", );
    for device in vec_devices {
        println!("\t Device {:?} : {:?}", device.name, device.desc)
    }
}

// Get request device
fn get_requested_device<'a> (requested_device_s : &str, requested_device : &'a mut Device, vec_devices : &'a Vec<Device>) {
    for device in vec_devices {
        if &*device.name == requested_device_s {
            requested_device.name = device.name.clone();
            requested_device.desc = device.desc.clone();
            println!("-{} device captured", requested_device_s);
        }
    }
}

fn main() {
    let mut requested_device : Device = Device::lookup().unwrap();

    let mut print_devices : bool = false;
    let mut requested_device_s : String = "wlp2s0".to_string();
    let mut verbose : bool = false;

    // What is the purpose of this block?
    {
        let mut argparse = ArgumentParser::new();
        argparse.set_description("Lets look at some packets");
        argparse.refer(&mut print_devices)
            .add_option(&["-p", "--print_devices"], StoreTrue, "Print devices found");
        argparse.refer(&mut requested_device_s)
            .add_option(&["-d", "--device"], Store, "Request a device");
        argparse.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], Store, "Be verbose");
        argparse.parse_args_or_exit();
    }

    let devices = Device::list();

    println!("requested_device : {}", requested_device_s);

    match devices {
        Ok(vec_devices) => {
            if print_devices {
                print_available_devices(&vec_devices);
                std::process::exit(0);
            }
            get_requested_device(&requested_device_s, &mut requested_device, &vec_devices);
        }
        Err(_) => {
            println!("No device found");
            std::process::exit(1);
        }
    }

    if requested_device_s != requested_device.name {
        std::process::exit(1);
    }

    println!("Default device, in case you are lost!! : {:?}", Device::lookup().unwrap().name);

    let mut cap = Capture::from_device(requested_device).unwrap().open().unwrap();

    // while let Ok(packet) = cap.next() {
    //     println!("received packet! {:?}", packet);
    // }

    // Relatively sure this is the number of packets received. Maybe we count bytes from packet.data
    // And channel that somewhere in a clever way
    while let Ok(stat) = cap.stats() {
        println!("..... {:?}", stat.received);
    }
}
