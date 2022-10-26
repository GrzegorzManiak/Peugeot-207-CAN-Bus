// sudo apt install pkg-config
// sudo apt install libudev-dev

use std::{io, time::Duration};

const BUAD_RATE: u32 = 115200;


fn main() {
    let ports = serialport::available_ports().expect("No ports found!");

    let mut i = 0;
    let mut ports_list = Vec::new();
    for p in ports {
        println!("[{}] {}", i, p.port_name);
        ports_list.push(p.port_name);
        i += 1;
    }


    // -- Debug port option
    println!("[{}] Debug Port", i);


    // -- Get user input
    println!("Select port: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let value = input.trim().parse::<usize>().unwrap();

     
    // -- Validate user input
    if value == i {
        println!("Opening debug port");
    }

    else if value > ports_list.len() {
        println!("Invalid port, exiting...");
        
        return;
    }
    else { 
        // -- Get port name
        let port_name = &ports_list[value];

        println!("Opening port: {} at {} Buad", port_name, BUAD_RATE);

        let port = serialport::new(port_name, BUAD_RATE)
            .timeout(Duration::from_millis(10))
            .open().expect("Failed to open port");
    }
}

