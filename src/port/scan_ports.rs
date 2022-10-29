use serialport::SerialPort;
use std::time::Duration;
use crate::port::test_port::test_port;

// @name scan_ports
// @desc Scan all available ports for a device
// @param baud_rate: u32 - The baud rate to use
// @return Option<Box<dyn SerialPort>>
pub fn scan_ports(buad_rate: u32) -> Option<Box<dyn SerialPort>> {
    // -- Get the ports
    let ports = serialport::available_ports().unwrap();

    // -- Loop through the ports
    for port in ports.iter() {
        // -- Attempt to open the port
        match serialport::new(port.port_name.clone(), buad_rate)
            .timeout(Duration::from_millis(10))
            .open() {
            
            // -- Opened the port successfully
            Ok(mut port) => {
                // -- Test the port
                if test_port(&mut port) {
                    // -- Return the port
                    return Some(port);
                }
            }

            // -- Welp, we couldn't open the port
            Err(e) => {
                // -- Print the error
                println!(">> Error: {}", e);
            }
        }
    }

    // -- No ports found
    println!(">> No ports found");

    // -- Return the none
    None
}