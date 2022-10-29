use super::{Packet, validate_raw_packet, destruct_packet_group};

// @name: return_packets
// @desc: Return the packets from the buffer
// @param: buffer: String - The buffer to read from
// @return: Vec<Packet>
pub fn return_packets(buffer: String) -> Vec<Packet> {
    // -- Create a vector to store the packets
    let mut packets = Vec::new();

    // -- Split the buffer on the packet separator
    let raw_packets = destruct_packet_group::destruct_packet_group(buffer);

    // -- Loop through the packets
    for raw_packet in raw_packets {
        // -- Verify the packet
        if let Some(packet) = validate_raw_packet::validate_raw_packet(raw_packet) {
            // -- Add the packet to the vector
            packets.push(packet);
        }
    }

    // -- Return the packets
    packets
}