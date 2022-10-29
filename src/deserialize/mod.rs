use crate::packet::Packet;
use std::collections::HashMap;

mod F220;
mod F0B6;

pub trait Frame {
    fn freshness(&self) -> usize;
    fn print(&self);
    fn name(&self) -> &str;
    fn json(&self) -> serde_json::Value;
}

pub type CacheMap = HashMap<String, usize>;

pub enum DataFreshness {
    Fresh,
    Stale
}

fn figure_freshness(id: usize, frame_name: &str, cache: CacheMap) -> DataFreshness {
    let mut data_freshness = DataFreshness::Stale;

    if let Some(data) = cache.get(&frame_name.to_string()) {
        if *data == id {
            data_freshness = DataFreshness::Fresh;
        }
    }

    data_freshness
}


pub fn interpret_packet(packet: Packet, cache: &mut CacheMap) -> Option<Box<dyn Frame>>{
    let frame = match packet.id.to_uppercase().as_str() 
    {
        "220" => F220::frame(packet),
        "0B6" => F0B6::frame(packet),
        _ => return None
    };

    figure_freshness(
        frame.freshness(), 
        frame.name(), 
        cache.clone()
    );

    return Some(frame);
}