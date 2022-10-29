#![allow(non_snake_case)]

use crate::packet::Packet;
use std::collections::HashMap;

mod F220;
mod F0B6;

pub type FreshnessMap = HashMap<String, usize>;
pub type FrameMap = HashMap<String, Box<dyn Frame>>;
pub enum DataFreshness { Fresh, Stale }

pub trait Frame {
    fn freshness(&self) -> usize;
    fn print(&self);
    fn name(&self) -> &str;
    fn json(&self) -> serde_json::Value;
    fn clone(&self) -> Box<dyn Frame>;
}



fn figure_freshness(frame: Box<dyn Frame>, freshness: &mut FreshnessMap) -> DataFreshness {

    // -- Try to get the freshness of the frame
    let frame_freshness = freshness.get(frame.name());

    // -- Check if the frame is fresh
    match frame_freshness {
        Some(freshness) => {
            if *freshness == frame.freshness() {
                DataFreshness::Fresh
            } 

            else {
                DataFreshness::Stale
            }
        },
        None => {
            DataFreshness::Stale
        }
    }
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
            // -- If the data in the cache is stale, update it
            frames.insert(frame.name().to_string(), frame.clone());
            cache.insert(frame.name().to_string(), frame.freshness());

            // -- Print out the super frame
            println!("{}", get_super(frames));
        },
        DataFreshness::Fresh => {
        }
    };


    // -- Return the frame
    Some(frame)
}

pub fn get_super(
    frames: &mut FrameMap
) -> String {
    // -- Merge the frames into a single JSON object
    let mut json = serde_json::json!({});

    for (_, frame) in frames {
        json[frame.name()] = frame.json();
    }

    // -- Return the JSON string
    json.to_string()
}
    