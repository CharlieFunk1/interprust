use std::net::IpAddr;
use std::{fs,env};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Strip {
    pub strip_num: u8,
    pub num_pixels: u16,
    pub start_pos: [u16; 2],
    pub angle: i16,
    pub length: u16,
    pub line_color: [u8; 3],
    pub zig_zags: u8,
    pub zag_distance: i16,
    pub ip: IpAddr
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub num_strips: u8,
    pub rust_path: String,
    pub brightness: u8,
    pub mode: u8,
    pub video_stream_ip: IpAddr
    
}


pub fn json_read() -> Vec<Strip> {
    let path = "/home/matrix/rust/interprust/stripdata.json";
    let data = fs::read_to_string(path).expect("read to string");
    let strips: Vec<Strip> = serde_json::from_str(&data).expect("from string");
    
    strips
}

pub fn json_read_config() -> Config {
    let path = "/home/matrix/rust/interprust/configdata.json";
    let data = fs::read_to_string(path).unwrap();
    let config: Config = serde_json::from_str(&data).unwrap();
    
    config
} 
//pub fn json_write(strips: &Vec<Strip>) {
//    let path = "./stripdata.json";
//    let serialized_strip = serde_json::to_string(&strips).unwrap();
//    fs::write(path, serialized_strip).unwrap();
//}
