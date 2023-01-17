use opencv::imgproc::COLOR_GRAY2RGB;
use opencv::imgproc::COLOR_BGR2GRAY;
use crate::modules::rgb::Rgbstrip;

use opencv::prelude::*;
use opencv::{highgui, videoio};
use std::net::IpAddr;
use std::path::Path;
use std::{fs, time, thread};
use std::time::Instant;


//Sets up opencv  for videos and returns cap
pub fn opencv_setup(video: String) -> (videoio::VideoCapture, String) {
    
    //Creates a window to display video output of video to be interpolated
    let window = "video";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE).expect("highgui named window failed");
    
    //Returns an instance of the video from file.
    (videoio::VideoCapture::from_file(&video, videoio::CAP_ANY).expect("failed to create cap"), window.to_string())
	
}

//Sets up opencv for camera
pub fn opencv_setup_camera() -> (videoio::VideoCapture, String) {
    
    //Creates a window for camera
    let window = "camera";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE).expect("highgui named window failed");
    
    //Returns an instance of a camera for input
    (videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("failed to create camera"), window.to_string())
	
}

pub fn opencv_setup_ip(video_stream_ip: IpAddr) -> (videoio::VideoCapture, String) {
    
    //Creates a window for camera
    let window = "ip";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE).expect("highgui named window failed");
    
    let http_url = String::from("http://".to_owned() + &video_stream_ip.to_string()[..] + ":8080/0");
    
    //Returns an instance of a camera for input
    (videoio::VideoCapture::from_file(&http_url, videoio::CAP_ANY).expect("failed to create camera"), window.to_string())
	
}




//Gets next video frame and retuns it
pub fn opencv_process_frame(cap: &mut videoio::VideoCapture) -> Mat {
    let mut frame = Mat::default();
    cap.read(&mut frame).expect("Failed to read cap");
    
    frame
}

//Displays video window and draws strip lines
pub fn opencv_draw_frame(mut frame: &mut Mat, all_rgb_strips: &Vec<Rgbstrip>, window: &String) {
    
    //This loop iterates through all led strips and draws the strip lines on the frame
    all_rgb_strips.into_iter().for_each(|strip| {
 	strip.draw_leds(&mut frame);
    });
    
    //Show the frame in video window
    if frame.size().expect("Failed to get frame size").width > 0 {
	highgui::imshow(&window, frame).expect("Error in highgui imshow");
    }
}



pub fn opencv_loop(all_rgb_strips: &mut Vec<Rgbstrip>, mode: &u8, video_stream_ip: &IpAddr) {
    
    //Video mode.  Reads from videos in video folder  
    if *mode == 1 {
	
	//Create path variable containng location to videos
	let path = Path::new("/home/matrix/rust/interprust/video");
	
	//Read the path for files
	for entry in fs::read_dir(path).expect("Unable to read directory") {
	    
	    //Get the files in path
	    let entry = entry.expect("Failed on result for entry");
	    
	    //Print curently playing entry
	    println!("{:?}", entry);
	    
	    //Runs process frame to update with new frame from video
	    let (mut cap, window) = opencv_setup(String::from(entry.path().into_os_string().into_string().expect("Failed to read video file in directory")));
	    
	    //Gets total and current frames numbers
	    let number_frames = cap.get(7).expect("Failed to get frame number");
	    let mut current_frame: f64 = 0.0;
	    
	    //Loops while the current frame is not the last frame
	    while current_frame < number_frames {
		let start = Instant::now();
		//Runs process frame to update with new frame from video
    		let mut frame: Mat = opencv_process_frame(&mut cap);
		//println!("1-{:?}", Instant::now() - start);
		//let cloned_tx = all_tx_and_num.clone();
		//Iterates through all rgb strips and sends updated frame
		let mut i = 1;
    		all_rgb_strips.into_iter().for_each(|strip| {
		    strip.update_config(i);
    		    strip.send(&frame);
		    i += 1;
    		});
		//println!("2-{:?}", Instant::now() - start);
		
		//Calls draw frame to display video window and draw lines
		opencv_draw_frame(&mut frame, &all_rgb_strips, &window);
		//println!("3-{:?}", Instant::now() - start);
		thread::sleep(time::Duration::from_millis(10));
		//println!("4-{:?}", Instant::now() - start);
	    
		current_frame += 1.0;
		let fps = Instant::now() - start;
		println!("fps={:?}", fps);
	    }
	}
    };
    //Camera using background subtraction
    if *mode == 2 {
	
	//Run opencv setup camera and create cap
	let (mut cap, window) = opencv_setup_camera();
	
	//Define some needed Mat items, a point, and a size type
	let mut fg_mask = Mat::default();
	let mut gray = Mat::default();
	let mut gray3c = Mat::default();
	let mut blur_frame = Mat::default();
	let point = opencv::core::Point {
	    x:5,
	    y:5
	};
	let size = opencv::core::Size2i::from_point(point);
	
	//Create backround subtraction mask
	let mut back_sub = opencv::video::create_background_subtractor_mog2(0, 0.0, false).expect("Error creating background subtractor");

	//Start the video loop
	loop {
	    
	    //Runs process frame to update with new frame from vide0
    	    let frame: Mat = opencv_process_frame(&mut cap);

	    //Convert to greyscale
	    opencv::imgproc::cvt_color(&frame, &mut gray, COLOR_BGR2GRAY, 3).expect("Failed to make image gray");
	    
	    //Add gausian blur
	    opencv::imgproc::gaussian_blur(&gray, &mut blur_frame, size, 5.0, 5.0, 0).expect("Failed to add gaussian blur");
	    
	    //Create background subtraction
	    opencv::prelude::BackgroundSubtractorMOG2::apply(&mut back_sub, &blur_frame, &mut fg_mask, 0.01).expect("Background subtraction has failed");

	    //Convert to 3 channel image
	    opencv::imgproc::cvt_color(&fg_mask, &mut gray3c, COLOR_GRAY2RGB, 3).expect("Failed to make image gray");

	    //Iterates through all rgb strips and sends updated frame
	    let mut i = 1;
    	    all_rgb_strips.into_iter().for_each(|strip| {
		strip.update_config(i);
    		strip.send(&gray3c);
		i += 1;
    	    });

	    //Calls draw frame to display video window and draw lines
	    opencv_draw_frame(&mut gray3c, &all_rgb_strips, &window);
 	    
	    //Time delay between frames
	    thread::sleep(time::Duration::from_millis(1));
	}
    };
    //Using background subtraction from static image (first frame)
    if *mode == 3 {
	//Setup camera and create cap
	let (mut cap, window) = opencv_setup_camera();

	//Declare vars
	let mut diff_frame = Mat::default();
	let mut thresh = Mat::default();
	let mut dilated = Mat::default();
	let mut gray = Mat::default();
	let mut gray3c = Mat::default();
   
	let struc_point = opencv::core::Point {
	    x: 0,
	    y: 0
	};
   
	let size = opencv::core::Size2i::from_point(struc_point);

	//Prepare reference image
	opencv::highgui::wait_key(1000).unwrap();
    	let ref_frame: Mat = opencv_process_frame(&mut cap);

	//Run loop
	loop {
	    //Read frame
    	    let frame: Mat = opencv_process_frame(&mut cap);
	    
	    //Apply absdiff.
	    opencv::core::absdiff(&ref_frame,
				  &frame,
				  &mut diff_frame).unwrap();
	    //Make greyscale
	    opencv::imgproc::cvt_color(&diff_frame,
				       &mut gray,
				       opencv::imgproc::COLOR_BGR2GRAY,
				       1).unwrap();
	    //apply gaussian blur
	    opencv::imgproc::gaussian_blur(&gray,
					   &mut dilated,
					   size,
					   5.0,
					   5.0,
					   0).unwrap();
	    //Apply threshold mask
	    opencv::imgproc::threshold(&dilated,
				       &mut thresh,
				       10.0,
				       255.0,
				       opencv::imgproc::THRESH_BINARY).unwrap();
	    
	    //Make greyscale 3 channel
	    opencv::imgproc::cvt_color(&thresh,
	    			       &mut gray3c,
	    			       opencv::imgproc::COLOR_GRAY2RGB,
	    			       3).unwrap();

	    //Iterates through all rgb strips and sends updated frame
    	    let mut i = 1;
    	    all_rgb_strips.into_iter().for_each(|strip| {
		strip.update_config(i);
    		strip.send(&gray3c);
		i += 1;
    	    });

	    //Calls draw frame to display video window and draw lines
	    opencv_draw_frame(&mut gray3c, &all_rgb_strips, &window);
 	    
	    //Time delay between frames
	    thread::sleep(time::Duration::from_millis(1));

	}
    };
    //IP mode.  Read video input from IP
if *mode == 4 {
	//Setup camera and create cap
	let (mut cap, window) = opencv_setup_ip(*video_stream_ip);

	//Declare vars
        let mut frame3 = Mat::default();
   
	//Run loop
	loop {
	    //Read frame
    	    let frame: Mat = opencv_process_frame(&mut cap);
	    //println!("{:?}", frame);
	    opencv::imgproc::cvt_color(&frame,
	    			       &mut frame3,
	    			       opencv::imgproc::COLOR_BGR2BGRA,
	    			       3).unwrap();

	    //Iterates through all rgb strips and sends updated frame
	    let mut i = 1;
    	    all_rgb_strips.into_iter().for_each(|strip| {
		strip.update_config(i);
    		strip.send(&frame3);
		i += 1;
    	    });

	    //Calls draw frame to display video window and draw lines
	    opencv_draw_frame(&mut frame3, &all_rgb_strips, &window);
 	    
	    //Time delay between frames
	    thread::sleep(time::Duration::from_millis(1));

	}
    };

}

