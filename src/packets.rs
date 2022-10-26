// Eg packet: (14c,5)0,0,0,0,80
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



/*
    @name 220
    @desc Door state
*/
#[derive(Debug)]
struct F220 {
    pub front_left_door: bool,
    pub front_right_door: bool,
    pub rear_left_door: bool,
    pub rear_right_door: bool,
    pub boot: bool,
}
fn f220 (frame: Packet) -> F220 {
    F220 {
        front_left_door: frame.data[0][0],
        front_right_door: frame.data[0][1],
        rear_left_door: frame.data[0][2],
        rear_right_door: frame.data[0][3],
        boot: frame.data[0][4],
    }
}



/*
    @name 0B6
    @desc Odometer + extra
*/
#[derive(Debug)]
struct F0B6 {
    pub rpm: usize,
    pub speed: usize,
    pub driven: usize,
    pub fuel_consumption_counter: usize,
}
fn f0B6 (frame: Packet) -> F0B6 {

    // -- Tachometer, 1-13
    let tachometer = frame.bit_range(1, 13);

    // -- Speed, 12, Bit 17 - 24
    let speed = frame.bit_range(17, 24);

    // -- Odometer, 28, bit 28 - 32
    let odometer = frame.bit_range(28, 32);

    // -- Fule consumption, bit 49 - 56
    let fuel_consumption = frame.bit_range(49, 56);


    F0B6 {
        rpm: Packet::arr_to_usize(tachometer) * 100,
        speed: Packet::arr_to_usize(speed),
        driven: Packet::arr_to_usize(odometer) * 100,
        fuel_consumption_counter: Packet::arr_to_usize(fuel_consumption) * 100,
    }
}




pub fn translate_packet(packet: Packet) {
    match packet.id.as_str() {
        "220" => {
            let f220 = f220(packet);
            println!("{:?}", f220);
        },
        "0B6" => {
            let f0B6 = f0B6(packet);
            println!("{:?}", f0B6);
        },
        _ => {}
    }
}