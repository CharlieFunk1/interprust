use crate::modules::json_rw::{Strip, Config, json_read, json_read_config, json_read_strip_xy};

use opencv::prelude::*;
use opencv::core::Point;
use opencv::core::Scalar;
use std::sync::mpsc;

const packet_size:usize = 450;
    
#[derive(Debug, Clone)]
pub struct Rgbstrip {
    pub strip_num: u8,               //number of led strips
    pub num_pixels: u16,              //number of pixels for this led strip
    pub start_pos: [u16; 2],       //starting position of strip (first led)
    pub angle: i16,                  //angle of led strip (from first led)
    pub length: u16,                 //length of led strip in screen pixels
    pub strip_xy: Vec<(u16, u16)>,   //Vector that contains xy mappings for all leds in strip
    pub line_color: [u8; 3],    //Color of the line drawn on screen for strip
    pub tx: mpsc::Sender<[u8; packet_size]>,            //Transmitter for sending to thread
    pub zig_zags: u8,                //Number of zig-zags in the strip. 1 for no zig-zags.
    pub zag_distance: i16,           //Distance between zags.  Negative values zag in other direction.
    pub brightness: f32              //Global strip brightness
}

impl Rgbstrip {
    //Creates a new instance of Rgbstrip and and loads the provided values into the structs fields.
    //This is used with a loop to populate the all_led_strips vector in main.rs to create and map the x/y
    //coordinates for all needed led strips.

    pub fn new(tx: std::sync::mpsc::Sender<[u8; packet_size]>, num_thread: usize) -> Rgbstrip {

	let strips: Vec<Strip> = json_read();
	let config: Config = json_read_config();
	let strip = &strips[num_thread];
	let strips_xy = json_read_strip_xy();
	let strip_xy = &strips_xy[num_thread];
	
	Rgbstrip {
	    strip_num: strip.strip_num,        
	    num_pixels: strip.num_pixels,      
	    start_pos: strip.start_pos,        
	    angle: strip.angle * -1,           
	    length: strip.length,              
	    strip_xy: strip_xy.to_vec(), //vec![(strip.start_pos[0], strip.start_pos[1])], 
	    line_color: strip.line_color,
	    tx,
	    zig_zags: strip.zig_zags,
	    zag_distance: strip.zag_distance,
	    brightness: (config.brightness as f32) / 100.0
        }
    }

    //Sends frame to strip's thread
    pub fn send(&self, frame: &Mat) {
	self.tx.send(self.get_rgb_strip(&frame)).unwrap();
    }

    //Gets led i's xy coordinates and assigns an RGB value to it based on it's position on the screen
    //and returns an RGB array representing that pixel.  
    pub fn get_rgb(&self, frame: &Mat, i: usize) -> [u8; 3] {
	//println!("strip_xy={:?}", self.strip_xy);
	let pixel: opencv::core::Vec3b = *frame.at_2d(
	    self.strip_xy[i].1 as i32,
	    self.strip_xy[i].0 as i32
	).expect("Failed to read strip_xy");
	let rgb: [u8; 3] = [(*pixel.get(2).expect("Failed to get pixel 0") as f32 * self.brightness) as u8, (*pixel.get(1).expect("Failed to get pixel 1") as f32 * self.brightness) as u8, (*pixel.get(0).expect("Failed to get pixel 2") as f32 * self.brightness) as u8];
	rgb
    }
    

    //Returns an array of rgb values for the strip
    pub fn get_rgb_strip(&self, frame: &Mat) -> [u8; packet_size] {
	let mut rgb_strip: [u8; packet_size] = [0; packet_size];
	let mut i: usize = 0;
	let mut j: usize = 0;
	let mut k: usize = 0;
	while i < self.num_pixels as usize {
	    while j < 3 {
		rgb_strip[k] = self.get_rgb(&frame, i)[j];
		k += 1;
		j += 1;
	    }
	    j = 0;
	    i += 1;
	}
	rgb_strip
    }
   
    //Sets all x/y coordinates for each led in a strip and saves them in RGBstrip's struct strip_xy vector element
    //pub fn set_strip(&mut self) {
    //	let mut j = 1;
    //	while j < (self.num_pixels + 1) {
    //	    let x = (((self.length as f32/self.num_pixels as f32) * j as f32) * (self.angle as f32).to_radians().cos()) + self.start_pos[0] as f32;
    //        let y = (((self.length as f32/self.num_pixels as f32) * j as f32) * (self.angle as f32).to_radians().sin()) + self.start_pos[1] as f32;
    //	    self.strip_xy.push((x as u16, y as u16));
    //	    j += 1;
    //	}
    //}
    
    //Sets all x/y coordinates for each led strip for a zig zag pattern.  
    //pub fn set_strip_zig_zag(&mut self) {
    //	let mut i: f32 = 1.0;
    //	let mut k: f32 = 1.0;
    //	let mut j: f32 = 1.0;
    //	let mut x_start = self.start_pos[0];
    //	let mut y_start = self.start_pos[1];
    //	while i <= self.zig_zags as f32 {
    //	    while j < (self.num_pixels as f32/self.zig_zags as f32) {
    //		let x = (((self.length as f32/(self.num_pixels as f32/self.zig_zags as f32) * (j) as f32) * (self.angle as f32).to_radians().cos()) * k) + x_start as f32;
    //		let y = (((self.length as f32/(self.num_pixels as f32/self.zig_zags as f32) * (j) as f32) * (self.angle as f32).to_radians().sin()) * k) + y_start as f32;
    //		self.strip_xy.push((x as u16, y as u16));
    //		//print!("set strip zig zag={:?}", self.strip_xy);
    //		j += 1.0;
    //	    }
    //	    if i == self.zig_zags as f32 {
    //		break;
    //	    }
    //	    k = k * -1.0;
    //	    x_start = (((self.zag_distance as f32) * ((self.angle as f32 - (-90.0)).to_radians().cos())) + (self.strip_xy[((j * i) - 1.0) as usize].0) as f32) as u16;
    //	    y_start = (((self.zag_distance as f32) * ((self.angle as f32 - (-90.0)).to_radians().sin())) + (self.strip_xy[((j * i) - 1.0) as usize].1) as f32) as u16;
    //	    self.strip_xy.push((x_start as u16, y_start as u16));
    //	    i += 1.0;
    //	    j = 1.0;
    //	}
    //}

     //Draws dots representing the interpolation points
    pub fn draw_leds(&self, frame: &mut Mat) {
	let mut i: usize = 0;
	while i < self.num_pixels as usize {
	    let start = Point {
		x: self.strip_xy[i].0 as i32,
		y: self.strip_xy[i].1 as i32,
	    };
	    let end = Point {
		x: self.strip_xy[i].0 as i32,
		y: self.strip_xy[i].1 as i32,
	    };
	    
	    //Converts rgb to hsv to input line colors to opencv for drawing
	    let scalar_color = Scalar::new(self.line_color[2] as f64, self.line_color[1] as f64, self.line_color[0] as f64,0.0);
	    
	    //Draws the line
	    opencv::imgproc::line(frame, start, end, scalar_color, 5, 8, 0).unwrap();
	    i += 1;
	}
    }

    pub fn update_config(&mut self, num_strip: usize) {
	let config: Config = json_read_config();
	let strips: Vec<Strip> = json_read();
	let strip = &strips[num_strip - 1];
	let strips_xy = json_read_strip_xy();
	let strip_xy = &strips_xy[num_strip - 1];
	self.brightness = (config.brightness as f32) / 100.0;
	self.num_pixels = strip.num_pixels;
	self.start_pos = strip.start_pos;
	self.angle = strip.angle * -1;           
	self.length = strip.length;              
	self.strip_xy = strip_xy.to_vec(); //vec![(strip.start_pos[0], strip.start_pos[1])]; 
	self.line_color = strip.line_color;
    }
}


