use serde_json::json;
use crate::packet::Packet;
use super::Frame;

/*
    @name F1A0
    @desc Custom, Roll and Pitch
*/
#[derive(Debug)]
pub struct F1A0 {
    pub roll: f32,
    pub pitch: f32,
}


impl Frame for F1A0 {
    // -- Used for determining if the packet has changed
    fn freshness(&self) -> usize { self.roll as usize }

    // -- Print the data in a nice format
    fn print(&self) {
        println!("Roll: {}", self.roll);
    }

    // -- Return the name of the frame &str
    fn name(&self) -> &str { "1A0" }

    // -- Return the data as json
    fn json(&self) -> serde_json::Value {
        json!({
            "roll": self.roll,
            "pitch": self.pitch
        })
    }

    fn clone(&self) -> Box<dyn Frame> {
        Box::new(F1A0 {
            roll: self.roll,
            pitch: self.pitch
        })
    }
}


pub fn frame (inc_frame: Packet) -> Box<dyn Frame> {
    let roll_bytes = inc_frame.bit_range(1, 16); 
    let pitch_bytes = inc_frame.bit_range(17, 32);
    
    let frame = F1A0 { 
        roll: Packet::arr_to_usize(roll_bytes) as f32 / 100.0,
        pitch: Packet::arr_to_usize(pitch_bytes) as f32 / 100.0
    };

    Box::new(frame)
}