use futures_util::stream::SplitSink;
use opencv::prelude::*;
mod config;
use config::config::{strip_config_data, NUM_STRIPS, Strip};
use opencv::core::Point;
use opencv::core::Scalar;
//use tokio::sync::mpsc::UnboundedSender;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::sync::mpsc;
use opencv::{
    highgui,
    prelude::*,
    Result,
    videoio,
};
use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use tungstenite::protocol::Message;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;

#[derive(Debug)]
pub struct Rgbstrip {
    pub strip_num: u8,               //number of led strips
    pub num_pixels: u8,              //number of pixels for this led strip
    pub start_pos: (u16, u16),       //starting position of strip (first led)
    pub angle: i16,                  //angle of led strip (from first led)
    pub length: u16,                 //length of led strip in screen pixels
    pub strip_xy: Vec<(u16, u16)>,   //Vector that contains xy mappings for all leds in strip
    pub line_color: (u8, u8, u8),    //Color of the line drawn on screen for strip
    pub tx: mpsc::Sender<Vec<(u8, u8, u8)>>,            //Transmitter for sending to thread
}

impl Rgbstrip {
    //Creates a new instance of Rgbstrip and and loads the provided values into the structs fields.
    //This is used with a loop to populate the all_led_strips vector in main.rs to create and map the x/y
    //coordinates for all needed led strips.

    pub fn new(tx: mpsc::Sender<Vec<(u8, u8, u8)>>, sock: SocketAddr) -> Rgbstrip {

	//println!("connecting to: {:?}", sock);
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
	self.tx.send(self.get_rgb_strip(frame)).unwrap();
    }

    //Gets led i's xy coordinates and assigns an RGB value to it based on it's position on the screen
    //and returns an RGB tuple representing that pixel.  
    pub fn get_rgb(&self, frame: &Mat, i: usize) -> (u8, u8, u8) {
	//println!("FRAME = {:?}", frame);
	let pixel: opencv::core::Vec3b = *frame.at_2d(  
	    self.strip_xy[i].0 as i32,
	    self.strip_xy[i].1 as i32
	).unwrap();
	//println!("PIXEL: {:?}", pixel);
	let rgb: (u8, u8, u8) = (*pixel.get(0).unwrap(), *pixel.get(1).unwrap(), *pixel.get(2).unwrap());
	rgb
    }
    
    //Returns a vector of rgb values for the strip
    pub fn get_rgb_strip(&self, frame: &Mat) -> Vec<(u8, u8, u8)> {
	let mut rgb_strip = Vec::new();
	let mut i: usize = 0;
	while i < self.num_pixels as usize {
	    rgb_strip.push(self.get_rgb(&frame, i));
	    i += 1;
	}
	//println!("STRIP: {:?}", rgb_strip);
	rgb_strip
    }
    
    //Sets all x/y coordinates for each led in a strip and saves them in Strip struct's strip_xy vector element
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

	let scalar_color = Scalar::new(self.line_color.2 as f64, self.line_color.1 as f64, self.line_color.0 as f64,0.0);
	
	opencv::imgproc::line(frame, start, end, scalar_color, 5, 8, 0).unwrap();
    }
}

#[derive(Debug)]
pub struct RgbstripSender {
    pub ip: SocketAddr,
    pub rx: mpsc::Receiver<Vec<(u8, u8, u8)>>,
    pub sender: UnboundedSender<Vec<(u8, u8, u8)>>
}

impl RgbstripSender {
    pub fn new(ip: SocketAddr, rx: mpsc::Receiver<Vec<(u8, u8, u8)>>) -> RgbstripSender {
	let (sender, _) = unbounded();
	RgbstripSender {
	    ip: ip,
	    rx: rx,
	    sender: sender,
	}
    }

    pub fn send(&self, payload: Vec<(u8 ,u8 ,u8)>, ws_sender: SplitSink<WebSocketStream<TcpStream>, Message>) {
	println!("heelo frum sand");
	// println!("Payload {:?}", payload);
	//let mut bin_payload = Vec::new();
	//for element in payload {
	//    bin_payload.push(element.0);
	//    bin_payload.push(element.1);
	//    bin_payload.push(element.2);
	//}
	//let binary_msg = Message::binary(bin_payload);
	//println!("{:?}", payload);

	// TODO: this is the wrong sender.  Use the stream
	//ws_sender.send(payload).unwrap();
    }
}

pub fn opencv_setup(video: String) -> (videoio::VideoCapture, String) {
     //Creates a window to display video output of video to be interpolated
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE).expect("highgui named window failed");
    //Creates an instance of the video called cap.  Specify video to be played here: 
    (videoio::VideoCapture::from_file(&video, videoio::CAP_ANY).expect("failed to create cap"), window.to_string())
}

pub fn opencv_process_frame(cap: &mut videoio::VideoCapture, all_rgb_strips: &Vec<Rgbstrip>, window: &String) -> Mat {
    let mut frame = Mat::default();
    cap.read(&mut frame).expect("Failed to read cap");
    // TESTING CODE: I DON'T DO ANYTHING
    //println!("{:?}", frame);
    // END TESTING CODE

    //This loop iterates through all led strips and calls send to send frame to thread for display and then
    // draw_strip to draw ech strip to the video output.
    for strip in all_rgb_strips {
 	strip.draw_strip(&mut frame);
    }

    //Show the frame
    if frame.size().expect("Failed to get frame size").width > 0 {
	highgui::imshow(&window, &mut frame).expect("Error in highgui imshow");
    }
    //Delay in showing frames
    let key = highgui::wait_key(30).expect("failed to let key");
    if key > 0 && key != 255 {
	panic!("User exited program");
	}
    frame

}

