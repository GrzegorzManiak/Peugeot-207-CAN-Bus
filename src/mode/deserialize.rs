use serialport::SerialPort;

use crate::EOL_CHAR;
use crate::packet::{
    read_until::read_until,
    return_packets::return_packets,
};

use crate::deserialize::{
    interpret_packet,
    FreshnessMap,
    FrameMap
};

pub fn mode_deserialize(port: Option<Box<dyn SerialPort>>, debug_packet: String) {
    let mut freshness: FreshnessMap = FreshnessMap::new();
    let mut cache: FrameMap = FrameMap::new();

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
                    
                    // -- Interpret the packet
                    interpret_packet(
                        packet, 
                        &mut freshness, 
                        &mut cache
                    );
                }
            }
        },

        // -- A Debug port was selected
        None => {
            // -- Parse the group of packets
            let packets = return_packets(debug_packet);

            // -- Print the packets
            for packet in packets {
                
                // -- Interpret the packet
                interpret_packet(
                    packet, 
                    &mut freshness,
                    &mut cache
                );
            }
        },
    }
}