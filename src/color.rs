use image::Rgb;
use palette::{FromColor, Lch, Srgb};
use std::collections::HashMap;

pub fn pixel_to_hex(p: &Rgb<u8>) -> usize {
    let mut result: usize = 0;
    result += (p[0] as usize) << 16;
    result += (p[1] as usize) << 8;
    result += p[2] as usize;
    return result;
}

pub fn hex_to_pixel(hex: &usize) -> Rgb<u8> {
    let r: u8 = (hex >> 16 & 0xff).try_into().unwrap();
    let g: u8 = (hex >> 8 & 0xff).try_into().unwrap();
    let b: u8 = (hex & 0xff).try_into().unwrap();
    let res: Rgb<u8> = Rgb((r, g, b).into());
    return res;
}

pub fn populate_hashmap(map: &mut HashMap<usize, usize>, key: usize) {
    let k = map.get_mut(&key);
    if k.is_some() {
        *k.unwrap() += 1;
    } else {
        map.insert(key, 1);
    }
}

pub fn get_analogus_color(p: &Rgb<u8>) -> Vec<Rgb<u8>> {
    let mut res: Vec<Rgb<u8>> = Vec::new();
    let convert: [f32; 3] = [
        p[0] as f32 / 255.0,
        p[1] as f32 / 255.0,
        p[2] as f32 / 255.0,
    ];
    let original_color: Srgb = Srgb::new(convert[0], convert[1], convert[2]);
    let mut lch_color: Lch = Lch::from_color(original_color);
    for _ in 0..5 {
        lch_color.hue += 60.0;
        //lch_color.hue += 30.0;
        let ret_color = Srgb::from_color(lch_color);
        res.push(Rgb((
            (ret_color.red * 255.0) as u8,
            (ret_color.green * 255.0) as u8,
            (ret_color.blue * 255.0) as u8,
        )
            .into()));
    }
    return res;
}

pub fn get_most_popular_color(map: &HashMap<usize, usize>, pop_val: usize) -> Option<usize> {
    let mut biggest_val: usize = 0;
    let mut biggest_key: Option<usize> = None;

    for (key, val) in map.iter() {
        let pixel_val = hex_to_pixel(key);
        let r = pixel_val[0];
        let g = pixel_val[1];
        let b = pixel_val[2];

        let max_val = *[r, g, b].iter().max().unwrap();
        let min_val = *[r, g, b].iter().min().unwrap();
        let diff = max_val - min_val;

        if diff < pop_val as u8 {
            continue;
        }

        if biggest_val < *val
            && (r < 210 || g < 210 || b < 210)
            && (r >= 120 || g >= 120 || b >= 120)
        {
            biggest_val = *val;
            biggest_key = Some(*key);
        }
    }
    return biggest_key;
}
