use ndarray::*;
use image::{RgbImage, ImageBuffer};
use std::io::Result;
struct Encoder {
    yiq_matrix : Array2<f32>,
    rgb_samples : Array3<f32>,
    scanline : Array2<f32>,
    //composite_samples : Array2<f32>,
    //yiq_samples : Array3<f32>,   
    //y_filtered : Array1<f32>,
    //i_filtered : Array1<f32>,
    //q_filtered : Array1<f32>,
    //i_cos : Array1<f32>,
    //q_sin : Array1<f32>,
    //chroma : Array1<f32>,
    
    //temp_row : Array1<f32>,
    //temp_row2 : Array1<f32>
}
impl Encoder {
    pub fn new() -> Self {
        Self {
            //from https://en.wikipedia.org/wiki/YIQ
            yiq_matrix : array![[0.299,  0.587,  0.114],
                                [0.596, -0.275, -0.321],
                                [0.212, -0.523,  0.311]];
            rgb_sampels : Array::zeros((480, 640, 3)),
            //composite_samples : Array::zeros((525, 800)),
            //yiq_samples : Array::zeros((480, 3, 640)),
            //y_filtered : Array::zeros(640),
            //i_filtered : Array::zeros(640),
            //q_filtered : Array::zeros(640),
            //i_cos : Array::zeros(640),
            //q_sin : Array::zeros(640),
            //chroma : Array::zeros(640),
            //scanline : Array2::zeros(3, 640),
            //temp_row : Array1::zeros(640),
            //temp_row2 : Array1::zeros(640)
        }
    }
    pub fn new_encode_image(&mut self, frame : &RgbImage) -> Option<> {
        let (width, height) = frame.dimensions();
        if width != 640 && height != 480 {
            return None;
        }
        let image_data = frame.as_raw();
        let pixel_array = ArrayView::from_shape((height, width, 3), &image_data).unwrap();
        Zip::from(&mut self.rgb_samples).and(pixel_array).for_each(|f, &c| {
            (c as f32)/255.0;
        });
        //For interlacing
        for field_no in 0..2 {
            for i in 0..240 {
                let line_no = ((i * 2) + field_no) as usize;
                let scanline = self.rgb_samples.slice(s![line_no, ..]);
                for color in 0..3 {
                    let mut color_line = self.scanline.slice_mut(s![color, ..]);
                    let color_vector = self.yiq_matrix.slice(s![color, ..]); //The weights used for the weighted average
                    Zip::from(&mut color_line).and(&scanline.outer_iter()).for_each(|s, &v| {
                        *s = color_vector.dot(v);
                    });
                }
                let y_slice = self.scanline.slice(s![0, ..]);
                let i_slice = self.scanline.slice(s![1. ..]);
                let q_slice = self.scanline.slice(s![2, ..]);
            }
        }
    }
}