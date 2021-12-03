use interprust::{RgbstripSender, Rgbstrip, opencv_setup, opencv_process_frame};
mod config;
use config::config::NUM_STRIPS;
use opencv::{
    highgui,
    prelude::*,
    Result,
    videoio,
};
use std::sync::mpsc;
use tokio;
use std::env;
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;


#[tokio::main]
async fn main() -> Result<()> {
    let (mut cap, window) = opencv_setup(String::from("/home/charlie/rust/interprust/video/newlong4.mov"));
    
    //TODO spawn threads for them.
    //TODO Additionally push Rgbstrip to vector outside of loop.
    //TODO Dynamic ip detection for server host.
    let addr = env::args().nth(1).unwrap_or_else(|| "192.168.0.18:8081".to_string());
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    
    let mut num_threads = 0;
    let mut all_rgb_strips : Vec<Rgbstrip> = Vec::new();
    while let Ok((stream, ip)) = listener.accept().await{ 
	println!("num_threads {}, NUM_STRIPS {}", num_threads, NUM_STRIPS);
	let (tx, rx) = mpsc::channel();
	let rgb_strip_sender = RgbstripSender::new(ip, rx);   // create rgbstripsender
	println!("rgbsender: {:?}", rgb_strip_sender);
	let mut rgbstrip = Rgbstrip::new(tx, ip);
	rgbstrip.set_strip();
	all_rgb_strips.push(rgbstrip);  // create rgbstrip and add to vector
	
	// spawn thread
	tokio::spawn(manage_connection(rgb_strip_sender));
	num_threads += 1;
	if num_threads >= NUM_STRIPS {
	    break
	}
    }
    
    loop {
	let mut frame: Mat = opencv_process_frame(&mut cap, &all_rgb_strips, &window);
	for strip in &all_rgb_strips {
	    strip.send(&frame);    
	}
    }
    Ok(())
}

//TODO Create handle connection.  Code that will be passed to each thread via a RgbstripSender object.
async fn manage_connection(rgb_strip_sender:RgbstripSender){
    //Receive payload from main
    loop {
	let payload = rgb_strip_sender.rx.recv().unwrap();
	rgb_strip_sender.send(payload);
    }
    //Send payload to strip
}
