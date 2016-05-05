#[macro_use(freenect_set_log_callback, freenect_set_depth_callback)]
extern crate freenect;
extern crate libc;
extern crate piston_window;
extern crate image as im;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod app;

use im::{ImageBuffer, Rgba};
use app::App;
use std::thread;
use std::sync::{Mutex, Arc};
use freenect::ffi::*;
use freenect::context::{Context, ContextDefault, StatusCode};
use freenect::device::{Device, DeviceDefault};
use freenect::buffer::{DepthBufferVideoMedium, Buffer};


fn main () {
    let mut context = Context::init (None).unwrap ();

    context.set_log_level (FreenectLogLevel::WARNING);
    freenect_set_log_callback! (context, fn cb (log_level : FreenectLogLevel, m : &str) {
        println! ("[LOG_LEVEL {:?}] {}", log_level, m);
    });
    context.select_subdevices(vec![FreenectDeviceFlags::CAMERA]);

    let n = context.num_devices ().unwrap ();
    println! ("Number of devices: {}", n);

    let mut dev = Device::open_device (&context, 0).unwrap ();
    let canvas  = Arc::new (Mutex::new (ImageBuffer::from_pixel (640, 480, Rgba([255; 4]))));

    dev.set_user_data (&mut canvas.clone ());



    dev.set_depth_mode (FreenectFrameMode::find_depth_mode (FreenectResolution::MEDIUM, FreenectDepthFormat::MM));



    freenect_set_depth_callback! (dev, fn depth_cb (array : &mut DepthBufferVideoMedium, timestamp: u32 ) {
       let canvas : &mut Arc<Mutex<ImageBuffer<Rgba<u8> ,Vec<u8>>>> = dev.get_user_data ();
       let mut canvas = canvas.lock ().unwrap ();
       let mut sum: f64=0 as f64;
       let size=640*480;
       for (data, pixel) in array.iter().zip (canvas.pixels_mut ()) {
        sum+=data.d as f64;
        let color=(255 as f64*(data.d as f64)/FREENECT_DEPTH_MM_MAX_VALUE as f64) as u8;
        *pixel =  Rgba ([color,color,color,255]);
    }
    let avg=sum/size as f64;
    println! ("avg in frame: {}",avg)
});


    dev.start_depth ();

    let new_ref = canvas.clone ();
    
    thread::spawn (|| {
        let mut app = App::new ();
        app.init (new_ref);
    });

    while context.process_events () == StatusCode::Success {
    }
}
