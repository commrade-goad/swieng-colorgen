mod color;
mod option;
use color::*;
use image::{DynamicImage, ImageReader, RgbImage};
use option::*;
use std::{collections::HashMap, fs};

fn main() {
    let arg = handle_args();
    if arg.is_some() {
        let prog_option = arg.unwrap();
        let mut map: HashMap<usize, usize> = HashMap::new();
        for i in 0..prog_option.file_path.len() {
            let current_path: &String = &prog_option.file_path[i];
            let img: DynamicImage = ImageReader::open(current_path).unwrap().decode().unwrap();
            match img {
                DynamicImage::ImageRgba8(_) => {
                    let _ = img.as_rgba8().unwrap();
                    todo!("PNG NOT SUPPORTED FOR NOW");
                }
                _ => {
                    let imgbuf = img.as_rgb8().unwrap();
                    for p in imgbuf.pixels() {
                        let convert = pixel_to_hex(p);
                        populate_hashmap(&mut map, convert);
                    }
                    let result = get_most_popular_color(&map, prog_option.prefer_pop_color);
                    if result.is_some() {
                        if !prog_option.output_file.is_empty() {
                            let ru = result.unwrap();
                            let mut lastnum: usize = 1;
                            let mut buffer: String = String::new();
                            buffer.push_str(&format!("bright01 = \"{:06x}\"\n", ru));
                            let res = get_closest_color(&hex_to_pixel(&ru));
                            for i in 0..res.len() {
                                let secbuffer: &str;
                                match i {
                                    0..=4 => secbuffer = "bright0",
                                    5..=10 => secbuffer = "dark0",
                                    11 | 13 | 15 => secbuffer = "black0",
                                    _ => secbuffer = "white0"
                                }
                                if i == 5 || i == 11 || i == 12 {
                                    lastnum = 1;
                                } else if i == 14 {
                                    lastnum = 2;
                                } else {
                                    lastnum += 1;
                                }
                                buffer.push_str(&format!("{}{} = \"{:06x}\"\n", secbuffer, lastnum, pixel_to_hex(&res[i])));
                            }
                            let _ = fs::write(prog_option.output_file, buffer.as_bytes());
                            return;
                        }
                        println!("{} : {:06x}", current_path, result.unwrap());
                        let res = get_closest_color(&hex_to_pixel(&result.unwrap()));
                        for i in 0..res.len() {
                            println!("{} : {:06x}", current_path,  pixel_to_hex(&res[i]));
                        }
                    }
                }
            }
        }
    }
}
