use std::net::IpAddr;
use std::collections::HashMap;

pub const NUM_STRIPS: u8 = 1;

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
pub fn strip_config_data() -> HashMap<IpAddr, Strip> {
   
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
	start_pos: (6,300),
	angle: 0,
	length: 700,
	line_color: (255,0,0),
    };
    let stripip1: IpAddr = "192.168.0.159".parse().unwrap(); 

    let strip2 = Strip {
	strip_num: 2,
	num_pixels: 8,
	start_pos: (6,400),
	angle: 0,
	length: 700,
	line_color: (0,0,255),
    };
    let stripip2: IpAddr = "192.168.0.140".parse().unwrap(); 
    
    let mut strip_config = HashMap::new();

    strip_config.insert(stripip1, strip1);  //Add each strip struct to list here with a push statement
    strip_config.insert(stripip2, strip2);
    
    return strip_config 
}

