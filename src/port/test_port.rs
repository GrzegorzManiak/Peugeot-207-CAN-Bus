use serialport::SerialPort;
use crate::{
    packet::read_until::read_until,
    FIN_PACKET, 
    ERR_PACKET, 
    ACK_PACKET,
    EOL_CHAR,
    MAX_RETRIES 
};

// Arduino > FIN
// Host > ACK
// Arduino > FIN


// @name: test_port
// @desc: Test a port to see if it is a valid port
// @param: port: &mut Box<dyn SerialPort> - The port to test
// @return: bool
// Basically, we send a message to the port and wait for a response
// if we get a response, we know the port is valid
pub fn test_port(port: &mut Box<dyn SerialPort>) -> bool {
    // -- Read the response
    let buffer: &mut Vec<u8> = &mut Vec::new();

    let mut fin_ack = false;

    // -- Read from the port
    for _ in 0..MAX_RETRIES {        
        let data = read_until(port, buffer, EOL_CHAR);

        // - Check if its an error
        if data.contains(ERR_PACKET) {
            return false;
        }

        // -- Check if the string contains the test message
        else if data.contains(FIN_PACKET) && fin_ack == false {
            // -- Respond with ACK
            port.write(format!("{}{}", ACK_PACKET, EOL_CHAR).as_bytes()).unwrap();
            port.write(format!("{}{}", FIN_PACKET, EOL_CHAR).as_bytes()).unwrap();

            // -- set fin_ack to true
            fin_ack = true;
        }

        // -- Check if the arduino responded with an ACK
        else if data.contains(ACK_PACKET) && fin_ack == true {
            // -- Respond with FIN
            port.write(format!("{}{}", FIN_PACKET, EOL_CHAR).as_bytes()).unwrap();

            // -- Return true
            return true;
        }
    }

    // -- If we didn't get a response, return false
    false
}