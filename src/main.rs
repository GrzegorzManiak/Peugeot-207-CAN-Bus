// sudo apt install pkg-config
// sudo apt install libudev-dev

mod packets;
use packets::Packet;

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
        let mut test_packet = "(14c,5)0,0,0,0,80 (168,8)0,0,0,0,0,0,0,0 (36,8)0,0,0,f,1,0,0,a0 (221,7)0,ff,ff,ff,ff,ff,ff (b6,8)0,0,0,0,0,0,0,d0 (227,4)0,0,0,0 (1a1,8)7f,ff,0,ff,ff,ff,ff,ff (18,5)80,0,0,0,0 (217,8)f2,80,0,0,80,ff,ff,ff (21f,3)0,0,0 (b6,8)0,0,0,0,0,0,0,d0 (167,8)8,6,ff,ff,7f,ff,0,0 (128,8)1,0,0,0,0,0,b0,1 (14c,5)0,0,0,0,80 (1a8,8)0,0,0,0,0,10,2b,2 ;";
        let packets = destruct_packet_group(test_packet.to_string());

        for packet in packets {
            packets::translate_packet(destruct_packet(packet));
        }
    }

    else if value > ports_list.len() {
        println!("Invalid port, exiting...");
        return;
    }
    else { 
        // -- Get port name
        let port_name = &ports_list[value];

        //  -- Open port
        println!("Opening port: {} at {} Buad", port_name, BUAD_RATE);
        let mut port = serialport::new(port_name, BUAD_RATE)
            .timeout(Duration::from_millis(10))
            .open().expect("Failed to open port");

        // -- Loop
        loop {
            // -- Read from serial
            let mut buf = [0; 128];

            match port.read(&mut buf) {
                Ok(t) => {
                    // -- If no data, continue
                    if t < 0 { return; }
                        
                    // -- Get data
                    let data = std::str::from_utf8(&buf).unwrap();

                    // -- Split data into packets
                    let packets = destruct_packet_group(data.to_string());

                    // -- Loop through packets
                    for packet in packets {
                        packets::translate_packet(destruct_packet(packet));
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }            
        }
    }
}


// Eg packet grouping: 
// (14c,5)0,0,0,0,80            (168,8)0,0,0,0,0,0,0,0 (36,8)0,0,0,f,1,0,0,a0   (18,5)80,0,0,0,0 
// (221,7)0,ff,ff,ff,ff,ff,ff   (b6,8)0,0,0,0,0,0,0,d0 (227,4)0,0,0,0           (1a1,8)7f,ff,0,ff,ff,ff,ff,ff
// (217,8)f2,80,0,0,80,ff,ff,ff (21f,3)0,0,0           (b6,8)0,0,0,0,0,0,0,d0   (167,8)8,6,ff,ff,7f,ff,0,0 
// (128,8)1,0,0,0,0,0,b0,1      (14c,5)0,0,0,0,80      (1a8,8)0,0,0,0,0,10,2b,2 ;
fn destruct_packet_group(packet: String) -> Vec<String> { 
    // Remove last ' ;' Characters from the packet
    let packet = packet.replace(" ;", "");

    // -- Destructure the packet
    let packets = packet.split(" ");

    // -- Create a vector to store the packets
    let mut packet_vec = Vec::new();

    // -- Loop through the packets
    for p in packets {
        packet_vec.push(p.to_string());
    }

    // -- Return the vector
    packet_vec
}

fn pad_string_to_len(string: String, len: usize, pad: String) -> String {
    let mut new_string = string;
    while new_string.len() < len {
        new_string = format!("{}{}", pad, new_string);
    }
    new_string
}

fn destruct_packet(packet: String) -> Packet {
    // -- Get the ID and Size '(ID,SIZE)DATA'
    let id_size = packet.split(")").next().unwrap();
    

    // -- Get the ID
    let id = id_size.split("(").nth(1)
        .unwrap()
        .split(",")
        .next()
        .unwrap()
        .to_string();

    // -- Get the size
    let size = id_size.split(",")
        .nth(1)
        .unwrap();

    // -- Get the data
    let data = packet.split(")").nth(1)
        .unwrap()
        .split(",")
        .map(|x| u8::from_str_radix(x, 16).unwrap())
        .map(|x| format!("{:08b}", x))
        .map(|x| x.chars()
            .map(|x| x == '1')
            .collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();


    // -- Return the packet
    Packet::new(
        pad_string_to_len(id.to_uppercase(), 3, "0".to_string()),
        u8::from_str_radix(size, 16).unwrap(),
        data
    )
}
