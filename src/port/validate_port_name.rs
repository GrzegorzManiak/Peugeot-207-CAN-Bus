// @name validate_port_name
// @desc Check if a port name is valid
// @param port: &str - The port name to check
// @return bool
pub fn validate_port_name(port_name: &str) -> bool {

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