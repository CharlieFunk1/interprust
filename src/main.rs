//TODO Make serial connection to microcontroller
//TODO Refine background subtraction more
//TODO Create way to have custom led layout (or more options) square?
//TODO Make it so microcontrollers can reconnect if connection lost and go blank when disconnect
//TODO make microcontrollers ip address detectable in gui

//TODO Make a config JSON with items from config options
//TODO make a way to save configs.  Perhaps using JSON saves?
//TODO Fix framerate of vidio_player output.
//TODO make it so you can upload video files to server

mod modules;

use modules::rgb::Rgbstrip;
use modules::opencv_func::opencv_loop;
use modules::json_rw::{Strip, Config, json_read, json_read_config};

use std::sync::mpsc;
use tokio;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::net::IpAddr;

    
#[tokio::main]
async fn main() {
    //Create num_threads variable to use later to know when we have received enough connections
    let mut num_threads = 0;

    //Get config info from json
    let config = json_read_config();
    let mode = config.mode;
    let video_stream_ip = config.video_stream_ip;
    
    //Create all_rgb_strips vector to hold all of the rgb strips
    let mut all_rgb_strips : Vec<Rgbstrip> = Vec::new();

    //Prepare ip list
    let strips: Vec<Strip> = json_read();
    let num_strips = strips.len();
    let mut iplist = Vec::new();
    for strip in strips {
	iplist.push(strip.ip);
    }
    
    //This is the loop that creates an thread for each strip and adds ip address and transmitter/reciever
    loop {
	let addr = String::from("192.168.1.112:808") + &num_threads.to_string();

	let ip = iplist[num_threads];
		
	//Initialize UDP socket
	let sock = UdpSocket::bind(addr).await.expect("Failed to bind UDP address");
		
	//Create transmitter and reciever to communicate to each strip's thread
	let (tx, rx) = mpsc::channel();
		
	// create rgbstrip, initialize it, and add to vector	
	let mut rgbstrip = Rgbstrip::new(tx, num_threads);
	rgbstrip.set_strip_zig_zag();
	all_rgb_strips.push(rgbstrip);
		
	// spawn thread
	tokio::spawn(manage_connection(rx, sock, ip));
	num_threads += 1;
		
	// once we've made enough threads, move along
	if num_threads >= num_strips {
	    break
	}
    }
    
    loop {
	opencv_loop(&all_rgb_strips, &mode, &video_stream_ip);
	}
}

    
//Code that will be passed to each thread via a RgbstripSender object.
async fn manage_connection(rx: mpsc::Receiver<[u8; 450]>, sock: UdpSocket, ip: IpAddr){
    println!("WebSocket connection established: {}", ip);
    
    //Split stream into sender
    loop {
	//Receive payload from main	
	let payload = rx.recv().expect("Failed to recieve payload");
	let sip = SocketAddr::new(ip, 4210);
	//println!("{:?}", payload);
	//Send payload UDP to strip
	sock.send_to(&payload, sip).await.expect("Failed to send payload to thread");
    }
}
