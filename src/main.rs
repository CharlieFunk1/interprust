use futures_util::{StreamExt, SinkExt};
use interprust::{Rgbstrip, opencv_setup, opencv_process_frame, opencv_draw_frame};
mod config;
use config::config::NUM_STRIPS;
use opencv::prelude::*;
use std::sync::mpsc;
use tokio;
use std::env;
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //Run opencv setup to initialize video capture.  Specify video to be played here:
    let (mut cap, window) = opencv_setup(String::from("/home/charlie/rust/interprust/video/newlong4.mov"));
    //Set address of the server and listen for incoming connections
    let addr = env::args().nth(1).unwrap_or_else(|| "192.168.0.188:8081".to_string());
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    //Create num_threads variable to use later to know when we have received enough connections
    let mut num_threads = 0;
    //Create all_rgb_strips vector to hold all of the rgb strips
    let mut all_rgb_strips : Vec<Rgbstrip> = Vec::new();
    //This is the loop that creates an thread for each strip and adds ip address and transmitter/reciever
    while let Ok((stream, ip)) = listener.accept().await{
	//Create transmitter and reciever to communicate to each strip's thread
	let (tx, rx) = mpsc::channel();
	
	// create rgbstrip, initialize it, and add to vector	
	let mut rgbstrip = Rgbstrip::new(tx, ip);
	rgbstrip.set_strip();
	all_rgb_strips.push(rgbstrip);  
	
	// spawn thread
	tokio::spawn(manage_connection(rx, stream, ip));
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
    }
}
    
//Code that will be passed to each thread via a RgbstripSender object.
async fn manage_connection(rx: mpsc::Receiver<Vec<u8>>, stream: TcpStream, ip: SocketAddr){
    // create stream to microcontroller and perform handshake
    let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Error during the websocket handshake occurred");
    	println!("WebSocket connection established: {}", ip);
    //Split stream into sender
    let (mut ws_sender, _) = ws_stream.split();
    loop {
	//Receive payload from main	
	let payload = rx.recv().unwrap();
	//Convert payload to binary message
	let mess = Message::Binary(payload);
	//send message
	ws_sender.send(mess).await.unwrap();
    }
}
