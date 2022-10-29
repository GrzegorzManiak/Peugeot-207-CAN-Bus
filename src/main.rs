mod packet;
use packet::Packet;

mod deserialize;
use deserialize::{interpret_packet, CacheMap};

use serialport::SerialPort;
use std::{io, time::Duration};


// -- Buad rate we should use
const BUAD_RATE: u32 = 115200;

// -- Valid characters that a packet can contain
const VALID_CHAR: [
    char; 
    19
] = ['(', ')', ',', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

// -- Test packet to be used with the debug port
const TEST_PACKET: &str = "
(b6,8)0,0,0,0,0,0,0
,d0(14c,5)0,0,0,0,0(217,8)f2,80,0,0,80,ff,ff,f
f(3a7,8)10,0,0,3,65,0,35,5(168,8)2,0,0,2,4,0,0
,0(36,8)0,0,80,f,1,0,0,a0(21f,3)0,0,0(b6,8)36,
36,36,36,36,36,36,d0(220,2)0,0(1a1,8)7f,ff,0,ff,ff,f
f,ff,ff(e6,6)10,0,0,0,0,69(167,8)0,6,ff,ff,7f,f
f,0,0(b6,8)0,0,0,0,0,0,0,d0(df,3)84,0,60(217,
8)f2,80,0,0,80,ff,ff,ff
";


const FIN_PACKET: &str = "@GrzegorzManiak/fin";
const ACK_PACKET: &str = "@GrzegorzManiak/ack";

// -- If the device failed to initiate MCP2515, This is the response
const ERR_PACKET: &str = "@GrzegorzManiak/err";


fn main() {
    // -- Prompt the user for the port to use
    let port = prompt_for_port();
    let mut cache: CacheMap = CacheMap::new();


    match port {
        // -- A real port was selected
        Some(_) => {
            let buffer: &mut Vec<u8> = &mut Vec::new();
            let mut port = port.unwrap();
            
            // -- Start the loop 
            loop {
                // -- Read the packet from the port
                let raw_data = read_until(&mut port, buffer, '\n');

                // -- Parse the group of packets
                let packets = return_packets(raw_data);

                // -- Print the packets
                for packet in packets {
                    
                    // -- Interpret the packet
                    let interpreted_packet = interpret_packet(packet, &mut cache);
                }
            }
        },

        // -- A Debug port was selected
        None => {
            // -- Parse the group of packets
            let packets = return_packets(TEST_PACKET.to_string());

            // -- Print the packets
            for packet in packets {
                
                // -- Interpret the packet
                let interpreted_packet = interpret_packet(packet, &mut cache);
            }
        },
    }
}


// @name: destruct_packet_group
// @desc: Takes a string of packets and returns a vector of strings
// @param: group - String of packets to be split
// @return: Vec<String> - Vector of packets
fn destruct_packet_group(group: String) -> Vec<String> { 
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


// @name: validate_raw_packet
// @desc: Takes a string and verifies that it is a valid packet
// @param: packet - String to be verified
// @return: Option<Packet> - Returns a packet if it is valid, otherwise returns None
fn validate_raw_packet(packet: String) -> Option<Packet> {
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
        data: data_split.to_vec(),
    })
}


// @name: read_until
// @desc: Read from a serial port until a character is found
// @param: port: &mut serialport::SerialPort - The port to read from
// @param: buffer: &mut Vec<u8> - If we find the character, we will return whatevers left in the buffer
// @param: character: char - The character to look for
// @return: String
//
// We only add data to the buffer if we have data leftover from the last read
// eg: if we read 'Hello' and the character is 'l', we will return 'Hello' 
// and the next read will be 'lo' + data from the port
fn read_until(port: &mut Box<dyn SerialPort>, buffer: &mut Vec<u8>, character: char) -> String {
    // -- Read from the port
    let mut data = vec![0; 128];
    match port.read(&mut data) {
        Ok(t) => {
            // -- If we have data, add it to the buffer
            if t > 0 {
                buffer.extend_from_slice(&data[0..t]);
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }

    // -- Convert the buffer to a string
    let string = String::from_utf8_lossy(&buffer).to_string();

    // -- If the string contains the character, return the string
    if string.contains(character) {
        // -- Get the index of the character
        let index = string.find(character).unwrap();

        // -- Get the string before the character
        let return_string = string[0..index].to_string();

        // -- Remove the string before the character from the buffer
        buffer.drain(0..index + 1);

        // -- Return the string
        return return_string;
    }

    // -- If we don't have the character, return an empty string
    String::new()
}


// @name: return_packets
// @desc: Return the packets from the buffer
// @param: buffer: String - The buffer to read from
// @return: Vec<Packet>
fn return_packets(buffer: String) -> Vec<Packet> {
    // -- Create a vector to store the packets
    let mut packets = Vec::new();

    // -- Split the buffer on the packet separator
    let raw_packets = destruct_packet_group(buffer);

    // -- Loop through the packets
    for raw_packet in raw_packets {
        // -- Verify the packet
        if let Some(packet) = validate_raw_packet(raw_packet) {
            // -- Add the packet to the vector
            packets.push(packet);
        }
    }

    // -- Return the packets
    packets
}


// @name: select_port
// @desc: Select a port from a list of ports, 
//      will loop until a valid port is selected
//      a vlaid port will always be available due
//      to the debug port.
// @return: Box<dyn SerialPort>
fn prompt_for_port() -> Option<Box<dyn SerialPort>> {
    // -- Get the ports
    let ports = serialport::available_ports().unwrap();

    // -- Create a vector to store the ports
    let mut port_vec = Vec::new();

    // -- Loop through the ports
    for (i, port) in ports.iter().enumerate() {
        // -- Add the port to the vector
        port_vec.push(port.clone());

        // -- Print the port
        println!(">> [{}]: {}", i, port.port_name);
    }

    
    // -- Print a debug port 
    println!(">> [{}]: Debug", port_vec.len());

    // -- Print an exit option
    println!(">> [{}]: Exit", port_vec.len() + 1);

    // -- Loop until a valid port is selected
    loop {
        // -- Get the input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // -- Try to parse the input
        if let Ok(index) = input.trim().parse::<usize>() {

            // -- Check if the index is valid
            if index < port_vec.len() {
                // -- Get the port name
                let port_name = port_vec[index].port_name.clone();

                // -- Attempt to open the port
                match serialport::new(port_name, BUAD_RATE)
                    .timeout(Duration::from_millis(10))
                    .open() {
                    
                    // -- Opened the port successfully
                    Ok(mut port) => {
                        // -- Test the port
                        if test_port(&mut port) {
                            // -- Return the port
                            return Some(port);
                        }

                        // -- Port failed the test
                        println!(">> Port failed the test");
                    }

                    // -- Welp, we couldn't open the port
                    Err(e) => {
                        // -- Print the error
                        println!(">> Error: {}", e);
                    }
                }
            }

            // -- Check if the index is the debug port
            if index == port_vec.len() {
                // -- Return the debug port
                return None;
            }

            // -- Check if the index is the exit option
            if index == port_vec.len() + 1 {
                // -- Quit the program
                std::process::exit(0);
            }

            println!(">> Pick another port");
        }
    }
}


// @name: test_port
// @desc: Test a port to see if it is a valid port
// @param: port: &mut Box<dyn SerialPort> - The port to test
// @return: bool
// Basically, we send a message to the port and wait for a response
// if we get a response, we know the port is valid
fn test_port(port: &mut Box<dyn SerialPort>) -> bool {
    // -- Send the test message
    port.write(FIN_PACKET.as_bytes()).unwrap();

    // -- Read from the port
    let mut data = vec![0; 128];
    match port.read(&mut data) {
        Ok(t) => {
            // -- If we have data, add it to the buffer
            if t > 0 {
                // -- Convert the buffer to a string
                let string = String::from_utf8_lossy(&data[0..t]).to_string();

                // - Check if its an error
                if string.contains(ERR_PACKET) {
                    return false;
                }

                // -- Check if the string contains the test message
                else if string.contains(ACK_PACKET) {
                    // -- Respond with ACK
                    port.write(ACK_PACKET.as_bytes()).unwrap();
                    
                    // -- Return true
                    return true;
                }
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }

    // -- If we didn't get a response, return false
    false
}