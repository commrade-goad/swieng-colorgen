mod color;
mod option;
use option::*;
use color::*;
use std::collections::HashMap;
use image::{DynamicImage, ImageReader, RgbImage};

fn main() {
    let arg = handle_args();
    if arg.is_some() {
        let prog_option = arg.unwrap();
        let mut map: HashMap<usize, usize> = HashMap::new();
        let img: DynamicImage = ImageReader::open(prog_option.file_path).unwrap().decode().unwrap();
        let imgbuf: &RgbImage = img.as_rgb8().unwrap();
        for p in imgbuf.pixels() {
            let convert = pixel_to_hex(p);
            populate_hashmap(&mut map, convert);
        }
        let result = get_most_popular_color(&map, prog_option.prefer_pop_color);
        if result.is_some() {
            println!("the most used color is : {:06x}", result.unwrap());
        }
    }
}
