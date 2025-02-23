use image::Rgb;
use palette::{FromColor, Hsl, Lch, SetHue, Srgb};
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
        .into());
}

fn default_color_hue_value(color: &str) -> f32 {
    match color {
        "red" => return 360.0,
        "yellow" => return 60.0,
        "green" => return 120.0,
        "cyan" => return 180.0,
        "blue" => return 240.0,
        "magenta" => 300.0,
        _ => return -1.0,
    }
}

fn check_color_type(color: Hsl) -> &'static str {
    let mut hue: f32 = color.hue.into_inner();
    hue = (hue + 360.0) % 360.0;

    if (hue >= 0.0 && hue <= 35.0) || (hue > 330.0 && hue <= 360.0) {
        return "red";
    } else if hue > 35.0 && hue <= 75.0 {
        return "yellow";
    } else if hue > 75.0 && hue <= 160.0 {
        return "green";
    } else if hue > 160.0 && hue <= 190.0 {
        return "cyan";
    } else if hue > 190.0 && hue <= 260.0 {
        return "blue";
    } else if hue > 260.0 && hue <= 330.0 {
        return "magenta";
    }
    return "";
}

pub fn get_closest_color_ver2(p: &Rgb<u8>) -> HashMap<String, Rgb<u8>> {
    let mut ret_val: HashMap<String, Rgb<u8>> = HashMap::new();
    let p_as_srgb: Srgb = Srgb::new(
        p[0] as f32 / 255.0,
        p[1] as f32 / 255.0,
        p[2] as f32 / 255.0,
    );
    let accent_lch: Lch = Lch::from_color(p_as_srgb);
    let mut accent_hsl: Hsl = Hsl::from_color(accent_lch);

    // push accent to the hashmap (we dont care if its dark)
    // if in the future we care just move this below the if check
    let accent_srgb: Srgb = Srgb::from_color(accent_hsl);
    ret_val.insert("accent".to_string(), srgb_2_rgb(&accent_srgb));

    // check the accent color (what color is it?)
    // if red then make it the red color, etc
    let accent_color_def: &str = check_color_type(accent_hsl);
    ret_val.insert(accent_color_def.to_string(), srgb_2_rgb(&accent_srgb));

    // calculate the diff between the default blue and the current_color
    // accent color clamp it to 7 degree max or min so it didnt impact
    // that much but if want to impact more just delete clamp
    // or increase it.
    let hue = (accent_hsl.hue.into_inner() + 360.0) % 360.0;
    let diff = hue - default_color_hue_value(accent_color_def);

    // if the base color to dark lighten the rest
    let minimal_light: f32 = 0.4;
    if accent_hsl.lightness < minimal_light {
        accent_hsl.lightness = minimal_light + 0.1;
    }

    // for loop to get color not black and white one
    const WHEEL: [&str; 6] = ["red", "yellow", "green", "cyan", "blue", "magenta"];
    for i in 0..WHEEL.len() {
        let current_color = WHEEL[i];
        if current_color == accent_color_def {
            continue;
        }
        let default_value = default_color_hue_value(current_color);
        accent_hsl.set_hue(default_value + diff);
        let accent_srgb: Srgb = Srgb::from_color(accent_hsl);
        ret_val.insert(current_color.to_string(), srgb_2_rgb(&accent_srgb));
    }

    // for loop to get color not black and white one
    // but darker version
    let lightness_value: f32 = accent_hsl.lightness - 0.1;
    let saturation_value: f32 = accent_hsl.saturation + 0.1;
    for i in 0..WHEEL.len() {
        let current_color = WHEEL[i];

        // build the name to be color_dark
        let mut color_name: String = current_color.to_string();
        color_name.push_str("_dark");

        let default_value = default_color_hue_value(current_color);
        accent_hsl.set_hue(default_value + diff);

        // use dim lightness value
        // and the saturation too
        accent_hsl.lightness = lightness_value;
        accent_hsl.saturation = saturation_value;

        let accent_srgb: Srgb = Srgb::from_color(accent_hsl);
        ret_val.insert(color_name, srgb_2_rgb(&accent_srgb));
    }
    
    // create the black color
    accent_hsl = Hsl::from_color(accent_lch);
    accent_hsl.lightness = 0.1;
    accent_hsl.saturation = 0.2;
    for i in 0..3 {
        let accent_srgb: Srgb = Srgb::from_color(accent_hsl);
        let name_buffer = format!("black{}", i);
        ret_val.insert(name_buffer, srgb_2_rgb(&accent_srgb));
        accent_hsl.lightness += 0.16;
    }

    // create the white color
    accent_hsl = Hsl::from_color(accent_lch);
    accent_hsl.lightness = 0.9;
    accent_hsl.saturation = 0.2;
    for i in 0..3 {
        let accent_srgb: Srgb = Srgb::from_color(accent_hsl);
        let name_buffer = format!("white{}", i);
        ret_val.insert(name_buffer, srgb_2_rgb(&accent_srgb));
        accent_hsl.lightness -= 0.1;
    }

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
