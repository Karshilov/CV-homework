extern crate cv;
use cv::*;

pub fn guassian_filter(source: &Mat, kernel_x: Vec<f64>, kernel_y: Vec<f64>, buffer: &[u8]) -> Mat {
    let rows = source.rows;
    let cols = source.cols;
    for row in 0..rows {
        
    }
    return Mat::from_buffer(rows, cols, source.cv_type(), buffer);
}