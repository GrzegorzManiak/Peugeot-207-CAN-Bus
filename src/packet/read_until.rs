use std::io;

use serialport::SerialPort;

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
pub fn read_until(port: &mut Box<dyn SerialPort>, buffer: &mut Vec<u8>, character: char) -> String {
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