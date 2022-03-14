use crate::modules::rgb::Rgbstrip;

use opencv::prelude::*;
use opencv::{highgui, videoio};
use std::path::Path;
use std::{fs, time, thread};



//Sets up opencv and returns cap
pub fn opencv_setup(video: String) -> (videoio::VideoCapture, String) {
     //Creates a window to display video output of video to be interpolated
    let window = video.as_str();
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE).expect("highgui named window failed");
   //Creates an instance of the video called cap. 
    (videoio::VideoCapture::from_file(&video, videoio::CAP_ANY).expect("failed to create cap"), window.to_string())
}

//Gets next video frame and retuns it
pub fn opencv_process_frame(cap: &mut videoio::VideoCapture) -> Mat {
    //let now = Instant::now();
    let mut frame = Mat::default();
    cap.read(&mut frame).expect("Failed to read cap");
    
    //println!("opencv_process_frame took {:?} secs",now.elapsed().as_millis());
    frame
    
}

//Displays video window and draws strip lines
pub fn opencv_draw_frame(mut frame: &mut Mat, all_rgb_strips: &Vec<Rgbstrip>, window: &String) {
    //This loop iterates through all led strips and draws the strip lines on the frame
    //let now = Instant::now();

    for strip in all_rgb_strips {
 	strip.draw_leds(&mut frame);
    }
    //Show the frame in video window
    if frame.size().expect("Failed to get frame size").width > 0 {
	highgui::imshow(&window, frame).expect("Error in highgui imshow");
    }
    //Delay in showing frames

    let key = highgui::wait_key(1).expect("failed to let key");
    if key > 0 && key != 255 {
    	panic!("User exited program");
    	}
    //println!("opencv_draw_frame took {:?} secs",now.elapsed().as_millis());

}

pub fn opencv_loop(all_rgb_strips: &Vec<Rgbstrip>) {
    let path = Path::new("/home/charlie/rust/interprust/video");
    for entry in fs::read_dir(path).expect("Unable to read directory") {
	let entry = entry.expect("Failed on result for entry");
	println!("{:?}", entry);
	let (mut cap, window) = opencv_setup(String::from(entry.path().into_os_string().into_string().expect("Failed to read video file in directory")));
	let number_frames = cap.get(7).unwrap();
	let mut current_frame: f64 = 0.0;
	//let (mut cap, window) = opencv_setup(String::from("./video/newlong4.mov"));
	while current_frame < number_frames {
	    //Runs process frame to update with new frame from video
    	    let mut frame: Mat = opencv_process_frame(&mut cap);
	    
	    //Iterates through all rgb strips and sends updated frame
    	    for strip in all_rgb_strips {
    		strip.send(&frame);
    	    }
	    
	    //Calls draw frame to display video window and draw lines
	    opencv_draw_frame(&mut frame, &all_rgb_strips, &window);
	    thread::sleep(time::Duration::from_millis(29));
	    
	    current_frame += 1.0;
	}
	
    }
}
