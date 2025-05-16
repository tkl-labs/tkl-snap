use std::time::Instant;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use image::ImageEncoder;
use scap::frame::convert_bgra_to_rgb;
use scap::{
    capturer::{Area, Capturer, Options, Point, Size},
    frame::Frame,
};
use serde::Serialize;
use std::io::Cursor;
use std::sync::Mutex;

#[derive(Default)]
pub struct GlobalState {
    captures: Mutex<Vec<Capturer>>,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "message")]
pub enum CaptureError {
    UnsupportedPlatform,
    PermissionDenied,
    //CaptureFailed(String),
    //EncodingError(String),
    FrameTypeMismatch,
}

pub fn get_image_inner() -> Result<String, CaptureError> {
    if !scap::is_supported() {
        return Err(CaptureError::UnsupportedPlatform);
    }

    if !scap::has_permission() {
        println!("Requesting permission...");
        if !scap::request_permission() {
            return Err(CaptureError::PermissionDenied);
        }
    }

    // Get recording targets
    // let targets = scap::get_all_targets();
    // println!("Targets: {:?}", targets);

    let capture_width: u32 = 2560;
    let capture_height: u32 = 1440;

    // All your displays and windows are targets
    // You can filter this and capture the one you need.

    // Create Options
    let options = Options {
        fps: 60,
        target: None, // None captures the primary display
        crop_area: Option::from(Area {
            origin: Point { x: 0.0, y: 0.0 }, // top left
            size: Size {
                width: capture_width as f64,
                height: capture_height as f64,
            }, // change to actual resolution
        }),
        show_cursor: false,
        show_highlight: false,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_1440p,
    };

    // Create Capturer

    let start = Instant::now();
    let mut capturer = Capturer::build(options).unwrap();
    let duration = start.elapsed();

    eprintln!("\n\nTime Taken to build capturer: {:?}\n\n", duration);

    let start = Instant::now();
    // Start Capture
    capturer.start_capture();

    // Get frame from capturer
    let frame_data = capturer.get_next_frame().unwrap();

    // Stop Capture
    capturer.stop_capture();
    let duration = start.elapsed();
    eprintln!("\n\nTime Taken to capture: {:?}\n\n", duration);

    let start = Instant::now();
    let output = match &frame_data {
        // convert to bgr frame
        Frame::BGRA(bgr_frame) => {
            // get rgb data from bgr frame
            let rgb_data = convert_bgra_to_rgb(bgr_frame.data.clone());

            // create a buffer for PNG data
            let mut buffer = Cursor::new(Vec::new());

            // create a PNG encoder to write to buffer
            let encoder = image::codecs::png::PngEncoder::new(&mut buffer);

            encoder
                .write_image(
                    &rgb_data,
                    capture_width,
                    capture_height,
                    image::ExtendedColorType::Rgb8,
                )
                .expect("Failed");

            let png_bytes = buffer.into_inner();
            Ok(BASE64.encode(png_bytes))
        }
        _ => Err(CaptureError::FrameTypeMismatch),
    };

    let duration = start.elapsed();
    eprintln!("\n\nTime Taken to generate frame: {:?}\n\n", duration);

    return output;
}
