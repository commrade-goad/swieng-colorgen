mod color;
mod option;
use color::*;
use image::{DynamicImage, ImageReader, Rgb};
use option::*;
use std::{collections::HashMap, fs};

fn main() {
    let arg = handle_args();
    if arg.is_some() {
        let prog_option = arg.unwrap();
        let mut map: HashMap<usize, usize> = HashMap::new();
        let current_path: &String = &prog_option.file_path;
        let img: DynamicImage = ImageReader::open(current_path).unwrap().decode().unwrap();
        let imgbuf = img.to_rgb8();
        for p in imgbuf.pixels() {
            let convert = pixel_to_hex(p);
            populate_hashmap(&mut map, convert);
        }
        let result = get_most_popular_color(&map, prog_option.prefer_pop_color);
        if result.is_some() {
            if !prog_option.output_file.is_empty() {
                let ru = result.unwrap();
                let mut buffer: String = String::new();
                let res: HashMap<String, Rgb<u8>> = get_closest_color_ver2(&hex_to_pixel(&ru));
                for (name, color) in res.iter() {
                    buffer.push_str(&format!("{} = \"{:06x}\"\n", name, pixel_to_hex(color)));
                }
                let _ = fs::write(prog_option.output_file, buffer.as_bytes());
                return;
            }
            let res: HashMap<String, Rgb<u8>> = get_closest_color_ver2(&hex_to_pixel(&result.unwrap()));
            for (name, color) in res.iter() {
                println!("{} = {:06x}", name, pixel_to_hex(color));
            }
        } else {
            eprintln!("ERROR: bad parameter cant found any color!");
        }
    }
}
