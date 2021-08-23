extern crate cv;
use cv::highgui::*;
use cv::imgcodecs::ImageReadMode;
use cv::*;
use hw1::rgb2grey;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: display_image ImageToLoadAndDisplay");
        std::process::exit(-1);
    }

    let mat = Mat::from_path(&args[1], ImageReadMode::Color).expect("Failed to read from path");

    if !mat.is_valid() {
        eprintln!("Could not open or find the image");
        std::process::exit(-1);
    }

    let tgt = rgb2grey(&mat);

    highgui_named_window("Display window", WindowFlag::Autosize).unwrap();
    mat.show("Display window", 0).unwrap();
}