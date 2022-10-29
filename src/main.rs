pub mod packet;
pub mod deserialize;
pub mod mode;

use mode::{
    deserialize::mode_deserialize,
};
use serialport::SerialPort;
use std::{io, time::Duration};

pub struct Args {
    pub baud_rate: u32,
    pub mode: u8,
    pub port: String,
    pub cli: bool,
    pub auto: bool,
}

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
    let args = lapp::parse_args("
        -b, --buad (default 115200) The baud rate to use
        -p, --port (default debug) The port to use (If -c is set, this will be ignored), eg: debug, /dev/ttyUSB0
        -m, --mode (default 0) [0] Deserialize mode, [1] Listener mode
        -c Enable the CLI
        -a Automatically find the port (Will ignore -p)
    ");

    let args = Args {
        baud_rate: args.get_integer("buad") as u32,
        mode: args.get_integer("mode") as u8,
        port: args.get_string("port"),
        cli: args.get_bool("c"),
        auto: args.get_bool("a"),
    };



    let port: Option<Box<dyn SerialPort>>;

    // -- If we are in CLI mode, run the CLI
    if args.cli {
        // -- Prompt the user for the port to use
        port = prompt_for_port(args.baud_rate);
    }

    else {
        // -- If we are in auto mode, find the port
        if args.auto {
            port = scan_ports(args.baud_rate);
        }

        // -- Check if the user specified a port
        else if args.port != "" {
            if valid_port_name(&args.port) == false {
                println!("Invalid port name");
                return;
            }

            // -- Open the port
            port = open_port(&args.port, args.baud_rate);
        }

        // -- If the user didn't specify a port, use the debug port
        else {
            port = None;
        }
    }

    
    match args.mode {
        0 => {
            // -- Deserialize mode
            mode_deserialize(port);
        }
        1 => {
            // -- Listener mode
            //listener(port);
        }
        _ => {
            println!(">> Invalid mode");
        }
    }
}


// @name: select_port
// @desc: Select a port from a list of ports, 
//      will loop until a valid port is selected
//      a vlaid port will always be available due
//      to the debug port.
// @return: Box<dyn SerialPort>
fn prompt_for_port(buad_rate: u32) -> Option<Box<dyn SerialPort>> {
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

fn valid_port_name(port_name: &str) -> bool {

    // -- Check if the port name is the debug port
    if port_name == "debug" {
        return true;
    }

    // -- Get the ports
    let ports = serialport::available_ports().unwrap();

    // -- Loop through the ports
    for port in ports.iter() {
        // -- Check if the port name is valid
        if port.port_name == port_name {
            return true;
        }
    }

    // -- Port name is not valid
    false
}


fn open_port(port_name: &str, buad_rate: u32) -> Option<Box<dyn SerialPort>> {
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


fn scan_ports(buad_rate: u32) -> Option<Box<dyn SerialPort>> {
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