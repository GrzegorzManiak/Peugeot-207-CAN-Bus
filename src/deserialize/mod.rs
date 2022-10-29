use crate::packet::Packet;
use std::collections::HashMap;

mod F220;
mod F0B6;

pub trait Frame {
    fn freshness(&self) -> usize;
    fn print(&self);
    fn name(&self) -> &str;
    fn json(&self) -> serde_json::Value;
    fn clone(&self) -> Box<dyn Frame>;
}


pub type FreshnessMap = HashMap<String, usize>;
pub type FrameMap = HashMap<String, Box<dyn Frame>>;
pub enum DataFreshness { Fresh, Stale }



fn figure_freshness(frame: Box<dyn Frame>, freshness: &mut FreshnessMap) -> DataFreshness {
    let mut data_freshness = DataFreshness::Stale;

    
    // -- Check if the frame is in the freshness map
    if let Some(freshness_value) = freshness.get(frame.name()) {
        // -- Check if the frame is fresh
        if frame.freshness() != *freshness_value {
            data_freshness = DataFreshness::Fresh;
        }
    }


    // -- If the frame is not in the freshness map, it is fresh
    // and we should add it to the map
    else {
        data_freshness = DataFreshness::Fresh;
        freshness.insert(frame.name().to_string(), frame.freshness());
    }


    data_freshness
}


pub fn interpret_packet(
    packet: Packet, 
    cache: &mut FreshnessMap,
    frames: &mut FrameMap
) -> Option<Box<dyn Frame>>
{
    // -- Check if we have a parser for this frame
    let frame = match packet.id.to_uppercase().as_str() 
    {
        "220" => F220::frame(packet),
        "0B6" => F0B6::frame(packet),
        _ => return None
    };


    // -- Check if the data is fresh
    match figure_freshness(
        frame.clone(), cache
    ) {
        DataFreshness::Stale => {
            frames.insert(frame.name().to_string(), frame.clone());
        },
        DataFreshness::Fresh => {}
    };


    // -- Return the frame
    Some(frame)
}