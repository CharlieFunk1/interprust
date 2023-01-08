//TODO Make serial connection to microcontroller

//TODO Refine background subtraction more
//TODO Make it so microcontrollers can reconnect if connection lost and go blank when disconnect
//TODO make microcontrollers ip address detectable in gui

//TODO Replace absolute directories in json_rw.rs
//TODO Fix framerate of vidio_player output.
//TODO make it so you can upload video files to server
//TODO Make video stream to running page

mod modules;

use modules::rgb::Rgbstrip;
use modules::opencv_func::opencv_loop;
use modules::json_rw::{Strip, json_read, json_read_config};

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
    let host_ip = config.host_ip;
    
    
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
	//let addr = String::from("192.168.1.112:808") + &num_threads.to_string();
	let addr = format!("{}:808{}", host_ip, num_threads );
	//print!("{}", addr);
	let ip = iplist[num_threads];
	//print!("{}",iplist[num_threads]);
	//Initialize UDP socket
	let sock = UdpSocket::bind(addr).await.expect("Failed to bind UDP address");
		
	//Create transmitter and reciever to communicate to each strip's thread
	let (tx, rx) = mpsc::channel();
		
	// create rgbstrip, initialize it, and add to vector
	let rgbstrip = Rgbstrip::new(tx, num_threads);
	//rgbstrip.set_strip_zig_zag();
	all_rgb_strips.push(rgbstrip);
	
		
	// spawn thread
	tokio::spawn(manage_connection(rx, sock, ip));
	num_threads += 1;
		
	// once we've made enough threads, move along
	if num_threads >= num_strips {
	    break
	}
    }
    //Main loop.  Function is in loop because when video is over needs to loop to itself to start next video
    loop {
	opencv_loop(&mut all_rgb_strips, &mode, &video_stream_ip);
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
	println!("{:?}", payload);
	//Send payload UDP to strip
	sock.send_to(&payload, sip).await.expect("Failed to send payload to thread");
    }
}

