//TODO Make it so microcontrollers can reconnect if connection lost
//TODO Refactor elements to own modules
//TODO create square module
//TODO create JSON interface for config
//TODO create web app for configuring to JSON


use interprust::{Rgbstrip};
mod config;
use config::config::{NUM_STRIPS, strip_config_data};
// use opencv_func::{opencv_setup, opencv_process_frame, opencv_draw_frame};
use crate::opencv_func;
use opencv::prelude::*;
use std::sync::mpsc;
use tokio;
use std::net::SocketAddr;
use tokio::time::sleep;
use std::time::Duration;
use tokio::net::UdpSocket;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    //Run opencv setup to initialize video capture.  Specify video to be played here:
    let (mut cap, window) = opencv_setup(String::from("/home/charlie/rust/interprust/video/newlong4.mov"));
    
    //Create num_threads variable to use later to know when we have received enough connections
    let mut num_threads = 0;
    
    //Create all_rgb_strips vector to hold all of the rgb strips
    let mut all_rgb_strips : Vec<Rgbstrip> = Vec::new();
    
    //This is the loop that creates an thread for each strip and adds ip address and transmitter/reciever
    loop {
	let addr = String::from("192.168.1.118:808") + &num_threads.to_string();
	
	//Initialize UDP socket
	let sock = UdpSocket::bind(addr).await.unwrap();
	let (_, ip_list) = strip_config_data();
	let ip = ip_list[num_threads];
	
	//Create transmitter and reciever to communicate to each strip's thread
	let (tx, rx) = mpsc::channel();
	
	// create rgbstrip, initialize it, and add to vector	
	let mut rgbstrip = Rgbstrip::new(tx, ip);
	rgbstrip.set_strip_zig_zag();
	all_rgb_strips.push(rgbstrip);
	
	// spawn thread
	tokio::spawn(manage_connection(rx, sock, ip));
	num_threads += 1;
	
	// once we've made enough threads, move along
	if num_threads >= NUM_STRIPS {
	    break
	}
    }
    
    //Loop that will run in main thread to process video frames
    loop {
	//Runs process frame to update with new frame from video
    	let mut frame: Mat = opencv_process_frame(&mut cap);
	
	//Iterates through all rgb strips and sends updated frame
    	for strip in &all_rgb_strips {
    	    strip.send(&frame);
    	}
	
	//Calls draw frame to display video window and draw lines
	opencv_draw_frame(&mut frame, &all_rgb_strips, &window);
	sleep(Duration::from_millis(29)).await;
    }
}
    
//Code that will be passed to each thread via a RgbstripSender object.
async fn manage_connection(rx: mpsc::Receiver<[u8; 900]>, sock: UdpSocket, ip: IpAddr){
    println!("WebSocket connection established: {}", ip);
    
    //Split stream into sender
    loop {
	//Receive payload from main	
	let payload = rx.recv().unwrap();
	let sip = SocketAddr::new(ip, 4210);
	
	//send message
	sock.send_to(&payload, sip).await.unwrap();
    }
}
