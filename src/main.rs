extern crate argparse;
extern crate pnet;

use argparse::{ArgumentParser, Store, StoreTrue};

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket, PacketSize};

// Print available network interfaces
fn print_available_interfaces () {
    println!("Available Network interfaces: ", );
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        println!("\t Interface - {:?}", interface.name)
    }
}

// Get request device
fn get_requested_interface (requested_interface : &str) -> pnet::datalink::NetworkInterface {
    let interface_names_match = |iface: &NetworkInterface| iface.name == requested_interface;
    datalink::interfaces()
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap()
}

fn main() {
    let mut print_devices : bool = false;
    let mut requested_interface_s : String = "wlp2s0".to_string();
    let mut verbose : bool = false;

    // What is the purpose of this block?
    {
        let mut argparse = ArgumentParser::new();
        argparse.set_description("Lets look at some packets");
        argparse.refer(&mut print_devices)
            .add_option(&["-p", "--print_devices"], StoreTrue, "Print devices found");
        argparse.refer(&mut requested_interface_s)
            .add_option(&["-d", "--device"], Store, "Request a device");
        argparse.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], Store, "Be verbose");
        argparse.parse_args_or_exit();
    }

    println!("requested_device : {}", requested_interface_s);

    if print_devices {
        print_available_interfaces();
        std::process::exit(0);
    }

    let interface = get_requested_interface(&requested_interface_s);

    let (mut _tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating a datalink channel: {}", e)
    };

    let mut iter = rx.iter();
    loop {
        match iter.next() {
            Ok(packet) => {
                let packet_size = packet.packet_size();
                let source = packet.get_source();
                println!("Packet size: {}", packet_size);
                println!("Packet source: {}", source);
            }
            Err(_) => {
                panic!("An error occured")
            }
        }
    }
}
