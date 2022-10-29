use std::{io, time::Duration};
use serialport::SerialPort;
use crate::port::test_port::test_port;

// @name: select_port
// @desc: Select a port from a list of ports, 
//      will loop until a valid port is selected
//      a vlaid port will always be available due
//      to the debug port.
// @return: Box<dyn SerialPort>
pub fn prompt_for_port(buad_rate: u32) -> Option<Box<dyn SerialPort>> {
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