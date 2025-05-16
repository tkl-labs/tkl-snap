use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use std::io::Cursor;
use image::{ImageEncoder};
use scap::{capturer::{Area, Point, Size, Capturer, Options}, frame::Frame};
use scap::frame::{convert_bgra_to_rgb};
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", content = "message")]
enum CaptureError {
    UnsupportedPlatform,
    PermissionDenied,
    //CaptureFailed(String),
    //EncodingError(String),
    FrameTypeMismatch,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_image() -> Result<String, CaptureError> {
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
            size: Size { width: capture_width as f64, height: capture_height as f64 }, // change to actual resolution
        }),
        show_cursor: false,
        show_highlight: false,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_1440p
    };

    // Create Capturer
    let mut capturer = Capturer::build(options).unwrap();

    // Start Capture
    capturer.start_capture();

    // Get frame from capturer
    let frame_data = capturer.get_next_frame().unwrap();

    // Stop Capture
    capturer.stop_capture();

    match &frame_data {
        // convert to bgr frame
        Frame::BGRA(bgr_frame) => {
            // get rgb data from bgr frame
            let rgb_data = convert_bgra_to_rgb(bgr_frame.data.clone());

            // create a buffer for PNG data
            let mut buffer = Cursor::new(Vec::new());

            // create a PNG encoder to write to buffer
            let encoder = image::codecs::png::PngEncoder::new(&mut buffer);

            encoder.write_image(&rgb_data, capture_width, capture_height, image::ExtendedColorType::Rgb8).expect("Failed");

            let png_bytes = buffer.into_inner();
            Ok(BASE64.encode(png_bytes))
        }
        _ => Err(CaptureError::FrameTypeMismatch),
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}