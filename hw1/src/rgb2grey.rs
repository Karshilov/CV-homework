extern crate cv;
use cv::CvType::*;
use cv::*;

fn weight_mean_average(b: u16, g: u16, r: u16) -> u16 {
    return ((0.3 * r as f64 + 0.59 * g as f64 + 0.11 * b as f64) as f64) as u16;
}

#[no_mangle]
pub fn rgb2grey(source: &Mat, buffer: &mut Vec<u8>) -> Mat {
    let img_type: cv::CvType = source.cv_type();
    match img_type {
        Cv8UC1 | Cv8SC1 | Cv16UC1 | Cv16SC1 => {
            return Mat::from_buffer(source.rows, source.cols, img_type, source.data());
        }
        Cv32SC3 | Cv32FC3 | Cv64FC3 | Cv8UC2 | Cv32SC1 | Cv32FC1 => {
            eprintln!("unsupported image type");
            std::process::exit(-1);
        }
        _ => {}
    }
    let data = source.data();
    let element_size = source.elem_size();
    let channel_size = source.elem_size1();
    // println!("{} {} {}", img_type as i32, channel_size, element_size);
    for i in 0..data.len() {
        match i % element_size {
            0 => match channel_size {
                1 => {
                    let result =
                        weight_mean_average(data[i] as u16, data[i + 1] as u16, data[i + 2] as u16);
                    buffer.push(result as u8);
                }
                2 => {
                    let result = weight_mean_average(
                        data[i] as u16 * 256 + data[i + 1] as u16,
                        data[i + 2] as u16 * 256 + data[i + 3] as u16,
                        data[i + 4] as u16 * 256 + data[i + 5] as u16,
                    );
                    buffer.push((result / 256) as u8);
                    buffer.push((result % 256) as u8);
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
        _ => Cv8UC1,
    };
    println!("{} {} {}", data[0], data[1], data[2]);
    return Mat::from_buffer(source.rows, source.cols, target_type, &buffer[..]);
}
