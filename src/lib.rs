use opencv::prelude::*;
mod config;
use config::config::{strip_config_data, Strip};
use opencv::core::Point;
use opencv::core::Scalar;
use std::net::SocketAddr;
use std::sync::mpsc;
use opencv::{
    highgui,
    videoio,
};

#[derive(Debug)]
pub struct Rgbstrip {
    pub strip_num: u8,               //number of led strips
    pub num_pixels: u8,              //number of pixels for this led strip
    pub start_pos: (u16, u16),       //starting position of strip (first led)
    pub angle: i16,                  //angle of led strip (from first led)
    pub length: u16,                 //length of led strip in screen pixels
    pub strip_xy: Vec<(u16, u16)>,   //Vector that contains xy mappings for all leds in strip
    pub line_color: (u8, u8, u8),    //Color of the line drawn on screen for strip
    pub tx: mpsc::Sender<Vec<u8>>,            //Transmitter for sending to thread
}

impl Rgbstrip {
    //Creates a new instance of Rgbstrip and and loads the provided values into the structs fields.
    //This is used with a loop to populate the all_led_strips vector in main.rs to create and map the x/y
    //coordinates for all needed led strips.

    pub fn new(tx: mpsc::Sender<Vec<u8>>, sock: SocketAddr) -> Rgbstrip {
	
	let strip_config_data = strip_config_data();
	let strip: &Strip;
	let ip = sock.ip();
	strip = strip_config_data.get(&ip).unwrap();
	
	Rgbstrip {
	    strip_num: strip.strip_num,        
	    num_pixels: strip.num_pixels,      
	    start_pos: strip.start_pos,        
	    angle: strip.angle * -1,           
	    length: strip.length,              
	    strip_xy: vec![strip.start_pos], 
	    line_color: strip.line_color,
	    tx: tx, 
        }
    }

    //Sends frame to strip's thread
    pub fn send(&self, frame: &Mat) {
	self.tx.send(self.get_rgb_strip(&frame)).unwrap();
    }

    //Gets led i's xy coordinates and assigns an RGB value to it based on it's position on the screen
    //and returns an RGB tuple representing that pixel.  
    pub fn get_rgb(&self, frame: &Mat, i: usize) -> (u8, u8, u8) {
	let pixel: opencv::core::Vec3b = *frame.at_2d(  
	    self.strip_xy[i].1 as i32,
	    self.strip_xy[i].0 as i32
	).unwrap();
	let rgb: (u8, u8, u8) = (*pixel.get(0).unwrap(), *pixel.get(1).unwrap(), *pixel.get(2).unwrap());
	rgb
    }

    //Returns a vector of rgb values for the strip
    pub fn get_rgb_strip(&self, frame: &Mat) -> Vec<u8> {
	let mut rgb_strip = Vec::new();
	let mut i: usize = 0;
	while i < self.num_pixels as usize {
	    rgb_strip.push(self.get_rgb(&frame, i).0);
	    rgb_strip.push(self.get_rgb(&frame, i).1);
	    rgb_strip.push(self.get_rgb(&frame, i).2);   
	    i += 1;
	}
	rgb_strip
    }
    
    //Sets all x/y coordinates for each led in a strip and saves them in RGBstrip's struct strip_xy vector element
    pub fn set_strip(&mut self) {
	let mut j = 1;
	while j < (self.num_pixels + 1) {
	    let x = (((self.length as f32/self.num_pixels as f32) * j as f32) * (self.angle as f32).to_radians().cos()) + self.start_pos.0 as f32;
            let y = (((self.length as f32/self.num_pixels as f32) * j as f32) * (self.angle as f32).to_radians().sin()) + self.start_pos.1 as f32;
	    self.strip_xy.push((x as u16, y as u16));
	    j += 1;
	}
    }
    //Draws a line representing a strip on the video window where its interpolating the pixels from in the
    //color specified in the Strip struct's line_color element
    pub fn draw_strip(&self, frame: &mut Mat) {
	let x_end = (((self.length as f32/self.num_pixels as f32) * self.num_pixels as f32) * (self.angle as f32).to_radians().cos()) + self.start_pos.0 as f32;
	let y_end = (((self.length as f32/self.num_pixels as f32) * self.num_pixels as f32) * (self.angle as f32).to_radians().sin()) + self.start_pos.1 as f32;

	let start = Point {
	    x: self.start_pos.0 as i32,
	    y: self.start_pos.1 as i32,
	};
	
	let end = Point {
	    x: x_end as i32,
	    y: y_end as i32,
	};
	//Converts rgb to hsv to input line colors to opencv for drawing
	let scalar_color = Scalar::new(self.line_color.2 as f64, self.line_color.1 as f64, self.line_color.0 as f64,0.0);
	//Draws the line
	opencv::imgproc::line(frame, start, end, scalar_color, 5, 8, 0).unwrap();
    }
}

//Sets up opencv and returns cap
pub fn opencv_setup(video: String) -> (videoio::VideoCapture, String) {
     //Creates a window to display video output of video to be interpolated
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE).expect("highgui named window failed");
    //Creates an instance of the video called cap. 
    (videoio::VideoCapture::from_file(&video, videoio::CAP_ANY).expect("failed to create cap"), window.to_string())
}
//Gets next video frame and retuns it
pub fn opencv_process_frame(cap: &mut videoio::VideoCapture) -> Mat {
    let mut frame = Mat::default();
    cap.read(&mut frame).expect("Failed to read cap");
    frame
    
}
//Displays video window and draws strip lines
pub fn opencv_draw_frame(mut frame: &mut Mat, all_rgb_strips: &Vec<Rgbstrip>, window: &String) {
    //This loop iterates through all led strips and draws the strip lines on the frame
    for strip in all_rgb_strips {
 	strip.draw_strip(&mut frame);
    }
    //Show the frame in video window
    if frame.size().expect("Failed to get frame size").width > 0 {
	highgui::imshow(&window, frame).expect("Error in highgui imshow");
    }
    //Delay in showing frames
    let key = highgui::wait_key(30).expect("failed to let key");
    if key > 0 && key != 255 {
	panic!("User exited program");
	}

}
