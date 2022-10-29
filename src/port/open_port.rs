use std::time::Duration;
use serialport::SerialPort;
use crate::port::test_port::test_port;

// @name open_port
// @desc Open a port
// @param port_name: &str - The name of the port to open
// @param baud_rate: u32 - The baud rate to use
// @return Option<Box<dyn SerialPort>>
pub fn open_port(port_name: &str, buad_rate: u32) -> Option<Box<dyn SerialPort>> {
    // -- Check if the port name is the debug port
    if port_name == "debug" {
        return None;
    }

    // -- Attempt to open the port
    match serialport::new(port_name, buad_rate)
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
            
            // -- Exit the program
            std::process::exit(0);
        }

        // -- Welp, we couldn't open the port
        Err(e) => {
            // -- Print the error
            println!(">> Error: {}", e);

            // -- Exit the program
            std::process::exit(0);
        }
    }
}