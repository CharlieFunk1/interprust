//TODO Make it so microcontrollers can reconnect if connection lost and go blank when disconnect
//TODO create square module
//TODO figure out playlist looping
//TODO create video input
//TODO make microcontrollers ip address detectable

mod modules;

use modules::rgb::Rgbstrip;
use modules::opencv_func::opencv_loop;
use modules::json_rw::{Strip, json_read};

use std::sync::mpsc;
use tokio;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    //Create num_threads variable to use later to know when we have received enough connections
    let mut num_threads = 0;
    
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
	let addr = String::from("192.168.1.118:808") + &num_threads.to_string();

	let ip = iplist[num_threads];
	
	//Initialize UDP socket
	let sock = UdpSocket::bind(addr).await.unwrap();
	
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
	opencv_loop(&all_rgb_strips);
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
