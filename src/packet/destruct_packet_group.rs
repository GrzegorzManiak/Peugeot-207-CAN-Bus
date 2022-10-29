// @name: destruct_packet_group
// @desc: Takes a string of packets and returns a vector of strings
// @param: group - String of packets to be split
// @return: Vec<String> - Vector of packets
pub fn destruct_packet_group(group: String) -> Vec<String> { 
    // -- Verify that theres at least 1 packet
    if group.contains("(") == false {
        return Vec::new();
    }

    // -- Combine into one string, remove any new lines
    let mut group = group.replace("\r", "");
    group = group.replace("\n", "");

    // -- Destructure the packet
    let packets = group.split("(");

    // -- Create a vector to store the packets
    let mut packet_vec = Vec::new();

    // -- Loop through the packets
    for p in packets {
        // -- Push the packet to the vector, and add the opening bracket
        packet_vec.push(format!("({}", p));
    }

    // -- Return the vector
    packet_vec
}