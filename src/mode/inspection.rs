use serialport::SerialPort;
use crate::packet::Packet;
use crate::{EOL_CHAR, Args};
use crate::packet::{
    read_until::read_until,
    return_packets::return_packets,
};

pub fn print_packet(packet: Packet) {
    let mut packet_string = "".to_string();

    // -- Print the packet ID
    println!("-----------[ID: {}]-----------", packet.id);

    // -- Print the packet bytes
    packet.bytes.iter().for_each(|b| {
        packet_string.push_str(&format!("{} ", b.to_string()));
    });

    println!("{} \n{}", 
        packet.text(),
        packet_string
    );

    // -- Convert to binary
    let mut binary_string = "".to_string();
    packet.data.iter().for_each(|b| {
        b.iter().for_each(|b| {
            // -- boolean to int
            binary_string.push_str(&format!("{}", *b as i32));
        });

        binary_string.push_str(" ");
    });
    println!("{}", binary_string,);

}

pub fn mode_inspector(port: Option<Box<dyn SerialPort>>, args: Args) {
    match port {
        // -- A real port was selected
        Some(_) => {
            let buffer: &mut Vec<u8> = &mut Vec::new();
            let mut port = port.unwrap();
            
            // -- Start the loop 
            loop {
                // -- Read the packet from the port
                let raw_data = read_until(&mut port, buffer, EOL_CHAR);

                // -- Parse the group of packets
                let packets = return_packets(raw_data);

                // -- Print the packets
                for packet in packets {

                    // -- Only print the packet if it is the one we are looking for
                    if packet.id == args.inspect {
                        print_packet(packet);
                    }
                }
            }
        },

        // -- A Debug port was selected
        None => {
            // -- Parse the group of packets 
            let packets = return_packets(args.debug_packet);

            // -- Print the packets
            for packet in packets {
                
                // -- Only print the packet if it is the one we are looking for
                if packet.id == args.inspect {
                    print_packet(packet);
                }
            }
        },
    }
}