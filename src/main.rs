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
                            let mut lastnum: usize = 0;
                            let mut buffer: String = String::new();
                            let mut res: Vec<Rgb<u8>> = Vec::new();
                            get_closest_color_ver2(&hex_to_pixel(&ru));
                            todo!("GUARDED TESTING STUFF RN");
                            res.push(hex_to_pixel(&ru));
                            res.append(&mut get_closest_color(&hex_to_pixel(&ru)));
                            for i in 0..res.len() {
                                let secbuffer: &str;
                                match i {
                                    0..=5 => secbuffer = "bright0",
                                    6..=11 => secbuffer = "dark0",
                                    12 | 14 | 16 => secbuffer = "black0",
                                    _ => secbuffer = "white0"
                                }
                                if i == 6 || i == 12 || i == 13 {
                                    lastnum = 1;
                                } else if i == 15 {
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
