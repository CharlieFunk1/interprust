use std::net::IpAddr;
use std::collections::HashMap;

pub const NUM_STRIPS: usize = 3;

pub struct Strip {
    pub strip_num: u8,
    pub num_pixels: u8,
    pub start_pos: (u16, u16),
    pub angle: i16,
    pub length: u16,
    pub line_color: (u8, u8, u8),
}

//Function to create a vector with the structs containing each strip's data
//to pass to main.rs in order to generate strip maps
pub fn strip_config_data() -> (HashMap<IpAddr, Strip>, Vec<IpAddr>) {
   
//Create a struct for each strip in array here and input strip's data.
//Format:
//
//  let strip1 = Strip1 {
//      strip_num: 1,                         //This is the strip's number.  Starts at 1,2.....
//      num_pixels: 150,                      //Number of pixels for this led strip
//      start_pos: (600,5),                   //Starting position of the strip on screen (first led)
//      angle: 0,                             //Angle of the led strip (from first led)
//      length: 700,                          //Length of led strip in screen pixels
//      line_color: (255,255,255)             //Color of the line drawn on screen for strip
//  };
//  Const stripip1 IpAddrV4 = IpAddrV4::new(Ipv4Addr::new(192,168,0,149),81);   
    
    let strip1 = Strip {
	strip_num: 1,
	num_pixels: 150,
	start_pos: (450,6),
	angle: -90,
	length: 200,
	line_color: (255,0,0),
    };
    let stripip1: IpAddr = "192.168.1.155".parse().unwrap(); 

    let strip2 = Strip {
	strip_num: 2,
	num_pixels: 150,
	start_pos: (400,6),
	angle: -90,
	length: 200,
	line_color: (0,0,255),
    };
    let stripip2: IpAddr = "192.168.1.149".parse().unwrap();

    let strip3 = Strip {
	strip_num: 3,
	num_pixels: 150,
	start_pos: (500,6),
	angle: -90,
	length: 200,
	line_color: (0,255,0),
    };
    let stripip3: IpAddr = "192.168.1.22".parse().unwrap(); 
    
    let mut strip_config = HashMap::new();
    let mut ip_list = Vec::new();

    strip_config.insert(stripip1, strip1);  //Add each strip struct to list here with a push statement
    ip_list.push(stripip1);
    strip_config.insert(stripip2, strip2);
    ip_list.push(stripip2);
    strip_config.insert(stripip3, strip3);
    ip_list.push(stripip3);
    
    return (strip_config, ip_list) 
}

