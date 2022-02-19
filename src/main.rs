//TODO Make it so microcontrollers can reconnect if connection lost
//TODO Refactor websockets stuff to own function possibly solution to above
//TODO Fix stuttering problem (probably packet or network related)

use futures_util::{StreamExt, SinkExt};
use interprust::{Rgbstrip, opencv_setup, opencv_process_frame, opencv_draw_frame};
mod config;
use config::config::{NUM_STRIPS, strip_config_data};
use opencv::prelude::*;
use std::sync::mpsc;
use tokio;
use std::env;
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;
use std::net::SocketAddr;
use tokio::time::sleep;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    //Run opencv setup to initialize video capture.  Specify video to be played here:
    let (mut cap, window) = opencv_setup(String::from("/home/charlie/rust/interprust/video/newlong4.mov"));
    //Set address of the server and listen for incoming connections
    //let addr = env::args().nth(1).unwrap_or_else(|| "192.168.1.118:8081".to_string());
    //let sock = UdpSocket::bind(&addr).await.unwrap();
    //let sock = UdpSocket::bind("192.168.1.118:8081").await.unwrap();
    //let listener = try_socket.expect("Failed to bind");
    //println!("Listening on: {}", addr);
    //Create num_threads variable to use later to know when we have received enough connections
    let mut num_threads = 0;
    //Create all_rgb_strips vector to hold all of the rgb strips
    let mut all_rgb_strips : Vec<Rgbstrip> = Vec::new();
    //This is the loop that creates an thread for each strip and adds ip address and transmitter/reciever
    let mut i: usize = 0;
    while i < NUM_STRIPS {
	let addr = String::from("192.168.1.118:808") + &i.to_string();
	let sock = UdpSocket::bind(addr).await.unwrap();
	let (_, ip_list) = strip_config_data();
	let ip = ip_list[i];
	//Create transmitter and reciever to communicate to each strip's thread
	let (tx, rx) = mpsc::channel();
	
	// create rgbstrip, initialize it, and add to vector	
	let mut rgbstrip = Rgbstrip::new(tx, ip);
	rgbstrip.set_strip();
	all_rgb_strips.push(rgbstrip);  
	// spawn thread
	tokio::spawn(manage_connection(rx, sock, ip));
	num_threads += 1;
	i += 1;
	// once we've made enough threads, move along
	if num_threads >= NUM_STRIPS {
	    break
	}
    };
    //Loop that will run in main thread to process video frames
    loop {
	//let now = Instant::now();
	//Runs process frame to update with new frame from video
    	let mut frame: Mat = opencv_process_frame(&mut cap);
	//Iterates through all rgb strips and sends updated frame
    	for strip in &all_rgb_strips {
    	    strip.send(&frame);
    	}
	//Calls draw frame to display video window and draw lines
	opencv_draw_frame(&mut frame, &all_rgb_strips, &window);
	//println!("Main loop took {:?} secs",now.elapsed().as_millis());
	sleep(Duration::from_millis(29)).await;
    }
}
    
//Code that will be passed to each thread via a RgbstripSender object.
async fn manage_connection(rx: mpsc::Receiver<Vec<u8>>, sock: UdpSocket, ip: IpAddr){
    //let addr = env::args().nth(1).unwrap_or_else(|| "192.168.1.118:8081".to_string());
    // create stream to microcontroller and perform handshake
    //let ws_stream = tokio_tungstenite::accept_async(stream)
    //        .await
    //        .expect("Error during the websocket handshake occurred");
    	println!("WebSocket connection established: {}", ip);
    //Split stream into sender
    //let (mut ws_sender, _) = ws_stream.split();
    loop {
	//let now = Instant::now();
	//Receive payload from main	
	let payload = rx.recv().unwrap();
	let mut payload_arr: [u8; 450] = [0; 450];
	let mut i = 0;
	while i < payload.len() {
	    payload_arr[i] = payload[i];
	    i += 1;
	}
	let sip = SocketAddr::new(ip, 4210);
	//Convert payload to binary message
	//let mess = Message::Binary(payload);
	//send message
	sock.send_to(&payload_arr, sip).await.unwrap();
	//println!("Thread loop took {:?} secs",now.elapsed().as_millis());
	//sleep(Duration::from_millis(29)).await;
    }
}
