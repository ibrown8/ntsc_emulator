use ndarray::*;
use image::{RgbImage, ImageBuffer};
use std::io::Result;
use crate::common::Y_FILTER;
//Most TVs used UV decoding instead of iq decoding
//https://forums.nesdev.org/viewtopic.php?t=7261&start=45
//Shader used to emulated the effects of NTSC video coding and decoding.
//This emulator uses an interlaced version version of VGA resolution.
const PI : f32 = 3.14159265358979323;
pub struct Encoder {
    yuv_matrix : Array2<f32>, //Matrix to store RGB to YUV conversion
    rgb_samples : Array3<f32>, //Should be (480, 640, 3) //VGA Resolution
    composite_samples : Array2<f32>, //Should be (525, 800) //Based on VGA
    yuv_samples : Array3<f32>,   //Should be (3, 480, 640) //Stores The YUV data in planar format
    y_plane : Array2<f32>,
    u_plane : Array2<f32>,
    v_plane : Array2<f32>,
    //Most NTSC Decoders Used YUV instead of YIQ, Many encoders did too.
    //See https://www.analog.com/media/en/technical-documentation/obsolete-data-sheets/710320677ad722.pdf
    y_filtered : Array1<f32>, //Should be (640)
    u_filtered : Array1<f32>, //Should be (640)
    v_filtered : Array1<f32>, //Should be (640)
    u_cos : Array1<f32>, //u_filtered * cos(w)
    v_sin : Array1<f32>, //v_filtered * sin(w)
    chroma : Array1<f32>, //u_cos + v_sin
    scanline : Array2<f32>, //y + chroma
    temp_row : Array1<f32>, //temporary
    temp_row2 : Array1<f32>, //temporary
    //cos_table : Array1<f32> //storing the cosine as a lookup_table
    //sin_table : Array1<f32> //storing the sine as a lookup_table
}
struct YUVPlanar {
    pub y : Array2<f32>, //Fixed point?
    pub u : Array2<f32>,
    pub v : Array2<f32>
}
impl Encoder {
    pub fn new() -> Self {
        Self {
            //from https://en.wikipedia.org/wiki/YIQ
            //https://en.wikipedia.org/wiki/Y%E2%80%B2UV
            yuv_matrix : array![[0.299,  0.587,  0.114],
                                [-0.147, -0.289, 0.436],
                                [0.615, -0.515,  -0.100]],
            rgb_samples : Array::zeros((480, 640, 3)),
            composite_samples : Array::zeros((525, 800)),
            y_plane : Array::zeros((480, 659)),
            u_plane : Array::zeros((480, 659)),
            v_plane : Array::zeros((480, 659)),
            yuv_samples : Array::zeros((3, 480, 659)), //Larger than 640 to account for the zeros needed for padding the filters.
            y_filtered : Array::zeros(659), //Larger than 640 to account for the filter delay
            u_filtered : Array::zeros(659), //Larger than 640 to account for the filter delay
            v_filtered : Array::zeros(659),
            u_cos : Array::zeros(640),
            v_sin : Array::zeros(640),
            chroma : Array::zeros(640),
            scanline : Array2::zeros((3, 640)),
            temp_row : Array1::zeros(640),
            temp_row2 : Array1::zeros(640)
        }
    }
  
    pub fn new_encode_image(&mut self, frame : &RgbImage) -> Option<()> {
        let (width, height) = frame.dimensions();
        //Make sure resolution is right
        if width != 640 || height != 480 {
            return None;
        }
        let image_data = frame.as_raw();
        let pixel_array = ArrayView::from_shape((height as usize, width as usize, 3 as usize), &image_data).unwrap();
        Zip::from(&mut self.rgb_samples).and(pixel_array).for_each(|f, &c| {
            *f = (c as f32)/255.0;
        });
        //For interlacing
        for field_no in 0..2 {
            for (line_no, scanline) in self.rgb_samples.outer_iter().enumerate().filter(|(line_no, scanline)| line_no % 2 == field_no) {
                {
                //We know it is safe because the each of the mutable YUV slices cannot overlap due to the constant indexing.
                    let mut y_line = self.y_plane.slice_mut(s![line_no, ..]);
                    let mut u_line = self.u_plane.slice_mut(s![line_no, ..]);
                    let mut v_line = self.v_plane.slice_mut(s![line_no, ..]);
                    Zip::from(&mut y_line).and(&mut u_line).and(&mut v_line).and(scanline.rows()).for_each(|y, u, v, sample|{
                        let r = sample[0];
                        let g = sample[1];
                        let b = sample[2];
                        *y =  0.299 * r + 0.587 * g + 0.114 * b;
                        *u = -0.147 * r - 0.289 * g + 0.436 * b;
                        *v =  0.615 * r - 0.515 * g - 0.100 * b;
                    });
                }
                let y_line = self.y_plane.slice(s![line_no, ..]);
                let u_line = self.u_plane.slice(s![line_no, ..]);
                let v_line = self.v_plane.slice(s![line_no, ..]);
                //TODO: Optimize this later
                //do the y filtering 
                //Convolution formula from wikipedia
                //The filters are symmetric so this can be simpler.
                for x in (0..640){
                    let mut sample = 0.0;
                    for i in (0..19){
                        sample += y_line[x + i] * Y_FILTER[i];
                    }
                    self.y_filtered[x] = sample;
                }
                let mut composite_line = self.composite_samples.slice_mut(s![line_no, ..]);
                Zip::from(&mut composite_line).and(&self.y_filtered).for_each(|comp, y|{
                    *comp = *y
                });
                //TODO: filter y, i and q
                /* Zip::from(&mut i_cos).and(i_filtered).and(cos_table).for_each(|x, &i_val, &cos|{
                    *x = i_val * cos;
                };
                Zip::from(&mut q_sin).and(q_filtered).and(sin_table).for_each(|y, &q_val, &sin|{
                    *y = q_val * sin;
                };
                Zip::from(&mut chroma).and(i_sin).and(q_sin).for_each(|chroma, &i, &q|{
                    *chroma = i + q;
                }; */
            }
        }
        return Some(());
    }
}
