use serde_json::json;
use crate::packet::Packet;
use super::Frame;

/*
    @name 0B6
    @desc Odometer + extra
*/
#[derive(Debug)]
pub struct F0B6 {
    pub rpm: usize,
    pub speed: usize,
    pub driven: usize,
    pub fuel_consumption_counter: usize,
}


impl Frame for F0B6 {
    // -- Used for determining if the packet has changed
    fn freshness(&self) -> usize {
        self.rpm + self.speed + self.driven + self.fuel_consumption_counter
    }

    // -- Print the data in a nice format
    fn print(&self) {
        println!("RPM: {}", self.rpm);
        println!("Speed: {}", self.speed);
        println!("Driven: {}", self.driven);
        println!("Fuel consumption counter: {}", self.fuel_consumption_counter);
    }

    // -- Return the name of the frame &str
    fn name(&self) -> &str {
        "0B6"
    }

    // -- Return the data as json
    fn json(&self) -> serde_json::Value {
        json!({
            "rpm": self.rpm,
            "speed": self.speed,
            "driven": self.driven,
            "fuel_consumption_counter": self.fuel_consumption_counter
        })
    }
}


pub fn frame (frame: Packet) -> Box<dyn Frame> {

    // -- Tachometer, 1-13
    // -- Speed, 12, Bit 17 - 24
    // -- Odometer, 28, bit 28 - 32
    // -- Fule consumption, bit 49 - 56

    let tachometer = frame.bit_range(1, 13);
    let speed = frame.bit_range(17, 24);
    let odometer = frame.bit_range(28, 32);
    let fuel_consumption = frame.bit_range(49, 56);

    let frame = F0B6 {
        rpm: Packet::arr_to_usize(tachometer) * 100,
        speed: Packet::arr_to_usize(speed),
        driven: Packet::arr_to_usize(odometer) * 100,
        fuel_consumption_counter: Packet::arr_to_usize(fuel_consumption) * 100
    };

    Box::new(frame)
}