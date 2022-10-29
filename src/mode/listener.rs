use std::collections::HashMap;
use serialport::SerialPort;
use crate::EOL_CHAR;
use crate::packet::Packet;
use crate::packet::{
    read_until::read_until,
    return_packets::return_packets,
};

pub type PacketMap = HashMap<String, Packet>;


// ETC:
// [ID] | [BYTES] | [TEXT] || [ID] | [BYTES] | [TEXT] //
// 220  |  0B 00  |  0     || 220  |  0B 00  |  0     //
// 0B6  |  0B     |  0     || 0B6  |  2B     |  0     //
pub fn draw_table(map: &mut PacketMap) {
    // -- Create a vector to store the packets
    let mut packets = Vec::new();

    let mut max_data = 0;

    // -- Loop through the packets
    for (_, packet) in map {
        // -- Add the packet to the vector
        packets.push(packet);
    }

    // -- Sort the packets
    packets.sort_by(|a, b| a.id.cmp(&b.id));

    // -- Print the packets
    for packet in packets {
        let mut packet_string = "".to_string();

        packet.bytes.iter().for_each(|b| {
            packet_string.push_str(&format!("{} ", b.to_string()));
        });

        // -- Get the max data length
        if packet_string.len() > max_data {
            max_data = packet_string.len();
        }

        println!("{} | {} | {}", 
            packet.id, 
            pad_string(packet_string, max_data),
            packet.text()
        );
    }
}

pub fn pad_string(string: String, length: usize) -> String {
    let mut padded_string = string.clone();
    
    // -- Calculate the padding, we need to make sure that 
    // we dont get a negative number
    let padding = if length > string.len() {
        length - string.len()
    } else {
        0
    };  

    // -- Add the padding
    for _ in 0..padding {
        padded_string.push(' ');
    }
    
    padded_string
}

pub fn mode_listener(port: Option<Box<dyn SerialPort>>, debug_packet: String) {
    let mut packet_map: PacketMap = PacketMap::new();

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
                    // -- Add the packet to the map
                    packet_map.insert(packet.id.clone(), packet);
                }

                // -- Draw the table
                draw_table(&mut packet_map);
            }
        },

        // -- A Debug port was selected
        None => {
            // -- Parse the group of packets
            let packets = return_packets(debug_packet);

            // -- Print the packets
            for packet in packets {
                
                // -- Add the packet to the map
                packet_map.insert(packet.id.clone(), packet);
            }

            // -- Draw the table
            draw_table(&mut packet_map);
        },
    }
}