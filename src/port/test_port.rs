use serialport::SerialPort;
use std::io;
use crate::{
    FIN_PACKET, 
    ERR_PACKET, 
    ACK_PACKET
};

// @name: test_port
// @desc: Test a port to see if it is a valid port
// @param: port: &mut Box<dyn SerialPort> - The port to test
// @return: bool
// Basically, we send a message to the port and wait for a response
// if we get a response, we know the port is valid
pub fn test_port(port: &mut Box<dyn SerialPort>) -> bool {
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