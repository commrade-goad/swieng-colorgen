use image::Rgb;
use palette::{FromColor, Hsl, Lch, Srgb};
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

fn srgb_2_rgb(srgb: &palette::rgb::Rgb) -> Rgb<u8> {
    return Rgb((
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8,
    )
        .into())
}

pub fn get_closest_color(p: &Rgb<u8>) -> Vec<Rgb<u8>> {
    let mut ret_val: Vec<Rgb<u8>> = Vec::new();
    let p_as_srgb: Srgb = Srgb::new(
        p[0] as f32 / 255.0,
        p[1] as f32 / 255.0,
        p[2] as f32 / 255.0,
    );
    let mut accent_lch_color: Lch = Lch::from_color(p_as_srgb);
    let mut accent_hsl_color: Hsl = Hsl::from_color(accent_lch_color);
    accent_hsl_color.lightness += 0.2;
    accent_lch_color = Lch::from_color(accent_hsl_color);

    // for bright and normal color
    for _ in 0..5 {
        accent_lch_color.hue += 60.0;
        let changed_color = Srgb::from_color(accent_lch_color);
        ret_val.push(srgb_2_rgb(&changed_color));
    }

    accent_lch_color = Lch::from_color(p_as_srgb);

    // for dim color
    let mut accent_hsl_color_dim = accent_hsl_color.clone();
    accent_hsl_color_dim.lightness -= 0.1;
    let ret_color = Srgb::from_color(accent_hsl_color_dim);
    ret_val.push(srgb_2_rgb(&ret_color));
    for _ in 0..5 {
        accent_lch_color.hue += 60.0;
        let mut changed_hsl_color: Hsl = Hsl::from_color(accent_lch_color);
        changed_hsl_color.lightness -= 0.1;
        let changed_color = Srgb::from_color(changed_hsl_color);
        ret_val.push(srgb_2_rgb(&changed_color));
    }

    // for black and white color more higher
    accent_hsl_color.lightness = 0.1;
    let black = Srgb::from_color(accent_hsl_color);
    ret_val.push(srgb_2_rgb(&black));
    accent_hsl_color.lightness = 0.9;
    let white = Srgb::from_color(accent_hsl_color);
    ret_val.push(srgb_2_rgb(&white));

    // for black and white color more lower
    accent_hsl_color.lightness = 0.25;
    let black = Srgb::from_color(accent_hsl_color);
    ret_val.push(srgb_2_rgb(&black));
    accent_hsl_color.lightness = 0.75;
    let white = Srgb::from_color(accent_hsl_color);
    ret_val.push(srgb_2_rgb(&white));

    // for the comment stuff
    accent_hsl_color.lightness = 0.4;
    let black = Srgb::from_color(accent_hsl_color);
    ret_val.push(srgb_2_rgb(&black));
    return ret_val;
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
