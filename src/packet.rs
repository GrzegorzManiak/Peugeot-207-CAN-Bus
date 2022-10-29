#[derive(Debug)]
pub struct Packet {
    pub id: String,
    pub size: u8,
    pub data: Vec<Vec<bool>>,
}

impl Packet {
    pub fn new(id: String, size: u8, data: Vec<Vec<bool>>) -> Packet {
        Packet {
            id,
            size,
            data,
        }
    }

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
}