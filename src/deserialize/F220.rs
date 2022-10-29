use serde_json::json;
use crate::packet::Packet;
use super::Frame;

/*
    @name 220
    @desc Door state
*/
#[derive(Debug)]
pub struct F220 {
    pub front_left_door: bool,
    pub front_right_door: bool,
    pub rear_left_door: bool,
    pub rear_right_door: bool,
    pub boot: bool,
}


impl Frame for F220 {
    // -- Used for determining if the packet has changed
    fn freshness(&self) -> usize {
        self.front_left_door as usize + self.front_right_door as usize + 
        self.rear_left_door as usize + self.rear_right_door as usize + self.boot as usize
    }

    // -- Print the data in a nice format
    fn print(&self) {
        println!("Front left door: {}", self.front_left_door);
        println!("Front right door: {}", self.front_right_door);
        println!("Rear left door: {}", self.rear_left_door);
        println!("Rear right door: {}", self.rear_right_door);
        println!("Boot: {}", self.boot);
    }

    // -- Return the name of the frame &str
    fn name(&self) -> &str {
        "220"
    }

    // -- Return the data as json
    fn json(&self) -> serde_json::Value {
        json!({
            "front_left_door": self.front_left_door,
            "front_right_door": self.front_right_door,
            "rear_left_door": self.rear_left_door,
            "rear_right_door": self.rear_right_door,
            "boot": self.boot
        })
    }
}


pub fn frame (frame: Packet) -> Box<dyn Frame> {
    let frame = F220 {
        front_left_door: frame.data[0][0],
        front_right_door: frame.data[0][1],
        rear_left_door: frame.data[0][2],
        rear_right_door: frame.data[0][3],
        boot: frame.data[0][4],
    };

    Box::new(frame)
}