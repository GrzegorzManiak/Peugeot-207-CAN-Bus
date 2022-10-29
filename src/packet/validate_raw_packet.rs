use crate::VALID_CHAR;
use super::Packet;

// @name: validate_raw_packet
// @desc: Takes a string and verifies that it is a valid packet
// @param: packet - String to be verified
// @return: Option<Packet> - Returns a packet if it is valid, otherwise returns None
pub fn validate_raw_packet(packet: String) -> Option<Packet> {
    // -- Check if the packet is valid
    for c in packet.chars() {
        if !VALID_CHAR.contains(&c) {
            return None;
        }
    }

    // -- Packet must contain a '(' and a ')'
    if !packet.contains('(') || !packet.contains(')') {
        return None;
    }

    // -- Try to get the ID and size, if it fails then the packet is invalid
    // split the packet on )
    let split = packet.split(')').collect::<Vec<&str>>();

    // -- Check if the segment contains a comma and a '('
    if !split[0].contains(',') || !split[0].contains('(') {
        return None;
    }

    // -- Split the segment on the comma
    let info_split = split[0].split(',').collect::<Vec<&str>>();

    // -- Get the ID and size
    let id = info_split[0].trim().trim_start_matches('(');
    let size = info_split[1].trim().parse::<u8>().unwrap();


    // -- that the data segment contains size - 1 commas
    if split[1].matches(',').count() != (size - 1) as usize {
        return None;
    }

    // -- Parse the data segment
    let bytes: Vec<String> = split[1].split(',').map(|s| s.trim().to_string()).collect();

    // -- Split the data segment on the commas
    let data_split = split[1].split(',')
        .map(|x| u8::from_str_radix(x, 16).unwrap())
        .map(|x| format!("{:08b}", x))
        .map(|x| x.chars()
            .map(|x| x == '1')
            .collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();


    // -- Check if the data segment contains the correct number of bytes
    if data_split.len() != size as usize {
        return None;
    }

    // -- Pad the ID with 0's, if needed
    let mut id = id.to_string();

    while id.len() < 3 {
        id = format!("0{}", id);
    }
    
    // -- Create a packet struct
    Some(Packet {
        id,
        size,
        bytes,
        data: data_split.to_vec(),
    })
}