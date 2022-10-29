pub mod destruct_packet_group;
pub mod return_packets;
pub mod validate_raw_packet;
pub mod read_until;


#[derive(Debug)]
pub struct Packet {
    pub id: String,
    pub size: u8,
    pub data: Vec<Vec<bool>>,
    pub bytes: Vec<String>
}

impl Packet {
    pub fn bit_range(&self, start: usize, end: usize) -> Vec<&bool> {
        let mut bits = Vec::new();
        let mut i = 0;

        for byte in &self.data {
            for bit in byte {
                if i >= start && i < end {
                    bits.push(bit);
                }
                i += 1;
            }
        }

        // -- Return Vec<bool>
        bits
    }

    pub fn arr_to_usize(data: Vec<&bool>) -> usize {
        let mut i = 0;
        let mut num = 0;
        for bit in data {
            if *bit {
                num += 2usize.pow(i);
            }
            i += 1;
        }
        num
    }

    // -- Converts the bytes to a UTF-8 string
    pub fn text(&self) -> String {
        let mut text = String::new();
        for byte in self.bytes.iter() {
            text.push_str(&String::from_utf8_lossy(&[u8::from_str_radix(byte, 16).unwrap()]));
        }
        text
    }
}