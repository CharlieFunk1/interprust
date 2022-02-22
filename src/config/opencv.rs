use opencv::prelude::*;
use opencv::{
    highgui,
    videoio,
};
//use interprust::Rgbstrip;

//Sets up opencv and returns cap
pub fn opencv_setup(video: String) -> (videoio::VideoCapture, String) {
     //Creates a window to display video output of video to be interpolated
    let window = "video capture";
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
