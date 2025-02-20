mod color;
mod option;
use color::*;
use image::{DynamicImage, ImageReader, RgbImage};
use option::*;
use std::collections::HashMap;

fn main() {
    let arg = handle_args();
    if arg.is_some() {
        let prog_option = arg.unwrap();
        let mut map: HashMap<usize, usize> = HashMap::new();
        for i in 0..prog_option.file_path.len() {
            let current_path: &String = &prog_option.file_path[i];
            let img: DynamicImage = ImageReader::open(current_path).unwrap().decode().unwrap();
            let imgbuf: &RgbImage = img.as_rgb8().unwrap();
            for p in imgbuf.pixels() {
                let convert = pixel_to_hex(p);
                populate_hashmap(&mut map, convert);
            }
            let result = get_most_popular_color(&map, prog_option.prefer_pop_color);
            if result.is_some() {
                println!("{} : {:06x}", current_path, result.unwrap());
                let res = get_analogus_color(&hex_to_pixel(&result.unwrap()));
                for i in 0..res.len() {
                    println!("{} : {:06x}", current_path,  pixel_to_hex(&res[i]));
                }
            }
        }
    }
}
