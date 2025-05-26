mod common;
mod encoder;
//mod decoder;
use image::io::Reader as ImageReader;
use crate::encoder::Encoder;
fn main() {
    let args : Vec<String> = std::env::args().collect();
    if(args.len() < 2){
        println!("Need an Argument");
    }
    else {
        let filepath = &args[1];
        let image = ImageReader::open(filepath).unwrap().decode().unwrap();
        let image_rgb8 = image.to_rgb8();
        let mut encoder : encoder::Encoder = Encoder::new();
        encoder.new_encode_image(&image_rgb8);
    }
}
