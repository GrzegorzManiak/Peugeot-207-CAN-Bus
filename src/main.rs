pub mod packet;
pub mod deserialize;
pub mod mode;
pub mod port;

use mode::{
    deserialize::mode_deserialize,
    listener::mode_listener,
    inspection::mode_inspector,
};
use port::{
    prompt_for_port::prompt_for_port,
    open_port::open_port,
    scan_ports::scan_ports,
    validate_port_name::validate_port_name
};
use serialport::SerialPort;

pub struct Args {
    pub baud_rate: u32,
    pub mode: u8,
    pub port: String,
    pub cli: bool,
    pub auto: bool,
    pub debug_packet: String,
    pub inspect: String,
}

// -- Valid characters that a packet can contain
const VALID_CHAR: [
    char; 
    19
] = ['(', ')', ',', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

const FIN_PACKET: &str = "@GrzegorzManiak/fin";
const ACK_PACKET: &str = "@GrzegorzManiak/ack";
const ERR_PACKET: &str = "@GrzegorzManiak/err";

const EOL_CHAR: char = ';';
const MAX_RETRIES: u16 = 255; // -- Max ammount of tries to open a port

const USAGE: &'static str = "
@GrzegorzManiak/Peugeot-207-CAN-Bus

Command line interface for the Peugeot 207 Confort CAN Bus 
(This canvery easily be adapted to work with other cars, 
but I don't have any other cars, just clone the repo and 
change the packets in the deserialize module)

Allowing you to do things like:
: Read and write to the confort CAN Bus
: Figure out what packets do what
: Launch without a CLI and use the program as a library
: Use the debug port to test packets

Usage:
    -b, --buad (default 115200) The baud rate to use
    -p, --port (default debug) The port to use (If -c is set, this will be ignored), eg: debug, /dev/ttyUSB0
    -m, --mode (default 0) [0] Deserialize mode, [1] Listener mode, [2] Inspector mode
    -c Enable the CLI
    -i, --inspect (default '000') The packet to inspect
    -a Automatically find the port (Will ignore -p)
    -d, --debug-packet (default 'none') Provide a packet to use with the debug port
";


fn main() {
    let args = lapp::parse_args(USAGE);
    let args = Args {
        baud_rate: args.get_integer("buad") as u32,
        mode: args.get_integer("mode") as u8,
        port: args.get_string("port"),
        cli: args.get_bool("c"),
        auto: args.get_bool("a"),
        debug_packet: args.get_string("debug-packet"),
        inspect: args.get_string("inspect"),
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
            if validate_port_name(&args.port) == false {
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
            mode_deserialize(port, args);
        }
        1 => {
            // -- Listener mode
            mode_listener(port, args);
        }
        2 => {
            // -- Inspector mode
            mode_inspector(port, args);
        }
        _ => {
            println!(">> Invalid mode");
        }
    }
}
