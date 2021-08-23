extern crate cv;
use cv::CvType::*;
use cv::*;
use std::ops::{Add, Mul};

fn weight_mean_average(r: u16, g: u16, b: u16) -> u16 {
    return ((0.3 * r as f64 + 0.59 * g as f64 + 0.11 * b as f64) / 3 as f64) as u16;
}

pub fn rgb2grey(source: &Mat) -> Mat {
    let img_type: cv::CvType = source.cv_type();
    match img_type {
        Cv8UC1 | Cv8SC1 | Cv16UC1 | Cv16SC1 | Cv32SC1 | Cv32FC1 => {
            return Mat::from_buffer(source.rows, source.cols, img_type, source.data());
        }
        _ => {}
    }
    let data = source.data();
    let mut grey = Vec::new();
    let element_size = source.elem_size();
    let channel_size = source.elem_size1();
    println!("{} {} {}", img_type as i32, channel_size, element_size);
    for i in 0..data.len() {
        match i % element_size {
            0 => match channel_size {
                1 => {
                    let result =
                        weight_mean_average(data[i] as u16, data[i + 1] as u16, data[i + 2] as u16);
                    grey.push(result as u8);
                }
                2 => {
                    let result = weight_mean_average(
                        data[i] as u16 * 256 + data[i + 1] as u16,
                        data[i + 2] as u16 * 256 + data[i + 3] as u16,
                        data[i + 4] as u16 * 256 + data[i + 5] as u16,
                    );
                    grey.push((result / 256) as u8);
                    grey.push((result % 256) as u8);
                }
                _ => {}
            },
            _ => {}
        }
    }
    let target_type = match img_type {
        Cv8UC3 => Cv8UC1,
        Cv8SC3 => Cv8SC1,
        Cv16UC3 => Cv16UC1,
        Cv16SC3 => Cv16SC1,
        Cv32SC3 => Cv32SC1,
        Cv32FC3 => Cv32FC1,
        Cv64FC3 => Cv64FC1,
        _ => Cv8UC1,
    };
    return Mat::from_buffer(source.rows, source.cols, target_type, &grey);
}
