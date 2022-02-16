use futures_util::{StreamExt, SinkExt};
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
use std::net::SocketAddr;
//use tokio::sync::mpsc::unbounded_channel;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::accept_async;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

//sdsd
#[tokio::main]
async fn main() -> Result<()> {
    let (mut cap, window) = opencv_setup(String::from("/home/charlie/rust/interprust/video/newlong4.mov"));

    // TODO: fix it.
    let addr = env::args().nth(1).unwrap_or_else(|| "192.168.0.188:8081".to_string());
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    
    let mut num_threads = 0;
    let mut all_rgb_strips : Vec<Rgbstrip> = Vec::new();
    while let Ok((stream, ip)) = listener.accept().await{
	
	println!("num_threads {}, NUM_STRIPS {}", num_threads, NUM_STRIPS);
	
	let (tx, rx) = mpsc::channel();
	
	let rgb_strip_sender = RgbstripSender::new(ip, rx);   // create rgbstripsender
	//println!("rgbsender: {:?}", rgb_strip_sender);
	
	let mut rgbstrip = Rgbstrip::new(tx, ip);
	rgbstrip.set_strip();
	//println!("rgbstripxy: {:?}", rgbstrip.strip_xy);
	all_rgb_strips.push(rgbstrip);  // create rgbstrip and add to vector
	
	// spawn thread
	tokio::spawn(manage_connection(rgb_strip_sender, stream, ip));
	num_threads += 1;
	// once we've made enough threads, move along
	if num_threads >= NUM_STRIPS {
	    break
	}
    }
    
    loop {
    	let frame: Mat = opencv_process_frame(&mut cap, &all_rgb_strips, &window);
    	for strip in &all_rgb_strips {
    	    strip.send(&frame);
    	}
    }
    Ok(())
}
    
//Code that will be passed to each thread via a RgbstripSender object.
async fn manage_connection(rgb_strip_sender: RgbstripSender, stream: TcpStream, ip: SocketAddr){
    // create stream to microcontroller and perform handshake
    // let ws_stream: tokio_tungstenite::WebSocketStream<TcpStream> = tokio_tungstenite::accept_async(stream)
    let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Error during the websocket handshake occurred");
    	println!("WebSocket connection established: {}", ip);
    //Receive payload from main
    let (mut ws_sender, _) = ws_stream.split();
    loop {
	let payload = rgb_strip_sender.rx.recv().unwrap();
	//println!("Manage connections: {:?}", payload);
	let mut singlemess = Vec::new();
	let mut i = 0;
	while i < 150 {
	    singlemess.push(payload[i].0);
	    singlemess.push(payload[i].1);
	    singlemess.push(payload[i].2);
	    i += 1;
	    
	}
	let mess = Message::Binary(singlemess);
	//rgb_strip_sender.send(payload, ws_sender);
	//WHAT GOES HERE TO SEND PAYLOAD????????????
	ws_sender.send(mess).await.unwrap();
	// rgb_strip_sender.send(payload);
    }
    //Send payload to strip
}
