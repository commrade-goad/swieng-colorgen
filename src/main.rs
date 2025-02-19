mod color;
use color::*;
use std::{collections::HashMap, env};
use image::{DynamicImage, ImageReader, RgbImage};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let img: DynamicImage = ImageReader::open(&args[1]).unwrap().decode().unwrap();
    let imgbuf: &RgbImage = img.as_rgb8().unwrap();
    for p in imgbuf.pixels() {
        let convert = pixel_to_hex(p);
        populate_hashmap(&mut map, convert);
    }
    let result = get_most_popular_color(&map);
    if result.is_some() {
        println!("the most used color is : {:06x}", result.unwrap());
    }
}
