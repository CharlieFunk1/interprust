use std::net::IpAddr;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Strip {
    pub strip_num: u8,
    pub num_pixels: u8,
    pub start_pos: [u16; 2],
    pub angle: i16,
    pub length: u16,
    pub line_color: [u8; 3],
    pub zig_zags: u8,
    pub zag_distance: u16,
    pub ip: IpAddr
}



pub fn json_read() -> Vec<Strip> {
    let path = "/home/charlie/rust/interprust/stripdata.json";
    let data = fs::read_to_string(path).unwrap();
    let strips: Vec<Strip> = serde_json::from_str(&data).unwrap();
    
    strips
}

//pub fn json_write(strips: &Vec<Strip>) {
//    let path = "./stripdata.json";
//    let serialized_strip = serde_json::to_string(&strips).unwrap();
//    fs::write(path, serialized_strip).unwrap();
//}
