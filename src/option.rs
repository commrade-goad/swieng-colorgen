use std::env;

pub struct ProgramOption {
    pub file_path: String,
    pub prefer_pop_color: usize,
    pub output_file: String,
    pub min_light: f32,
    pub max_sat: f32,
    pub max_diff: f32,
}

impl ProgramOption {
    pub fn default_value() -> ProgramOption {
        return ProgramOption {
            file_path: String::new(),
            prefer_pop_color: 55,
            output_file: String::new(),
            min_light: 0.5,
            max_sat: 0.45,
            max_diff: 30.0,
        };
    }
}

fn print_help() {
    println!("USAGE : swieng-colorgen ./img.png [-flag]");
    println!("\t `-o`    | `--output` -> output as file need second args for the save.");
    println!("\t `-mpc`  | `--min-pop-color` -> what is the min rgb val diff to use as the accent. (default 55)");
    println!("\t `-minl` | `--min-light` -> what is the min light of the other color before its too dark. (default 0.5)");
    println!("\t `-maxs` | `--max-saturation` -> what is the max saturation value. (default 0.45)");
    println!("\t `-maxd` | `--max-diff` -> what is the max diff of hue so color dont bleed to ther color. (default 30.0)");
    std::process::exit(1);
}

pub fn handle_args() -> Option<ProgramOption> {
    let args: Vec<String> = env::args().collect();
    let mut prog_option: ProgramOption = ProgramOption::default_value();
    let mut idx = 1;
    while idx < args.len() {
        match &args[idx][..] {
            "--output" | "-o" => {
                if idx + 1 < args.len() {
                    prog_option.output_file = args[idx+1].clone();
                }
            },
            "--min-pop-color" | "-mpc" => {
                if idx + 1 < args.len() {
                    let mut def_val = false;
                    let pop_color = args[idx + 1].trim().parse().map_err(|e| {
                        eprintln!("ERROR: bad args : {}", e);
                        def_val = true;
                    });
                    if !def_val {
                        prog_option.prefer_pop_color = pop_color.unwrap();
                    }
                    idx += 1;
                }
            }
            "--min-light" | "-minl" => {
                if idx + 1 < args.len() {
                    let mut def_val = false;
                    let min_light = args[idx + 1].trim().parse().map_err(|e| {
                        eprintln!("ERROR: bad args : {}", e);
                        def_val = true;
                    });
                    if !def_val {
                        prog_option.min_light = min_light.unwrap();
                    }
                    idx += 1;
                }
            },
            "--max-saturation" | "-maxs" => {
                if idx + 1 < args.len() {
                    let mut def_val = false;
                    let max_sat = args[idx + 1].trim().parse().map_err(|e| {
                        eprintln!("ERROR: bad args : {}", e);
                        def_val = true;
                    });
                    if !def_val {
                        prog_option.max_sat = max_sat.unwrap();
                    }
                    idx += 1;
                }
            },
            "--max-diff" | "-maxd" => {
                if idx + 1 < args.len() {
                    let mut def_val = false;
                    let max_diff = args[idx + 1].trim().parse().map_err(|e| {
                        eprintln!("ERROR: bad args : {}", e);
                        def_val = true;
                    });
                    if !def_val {
                        prog_option.max_diff = max_diff.unwrap();
                    }
                    idx += 1;
                }
            },
            "--help" | "-h" => {
                print_help();
            },
            _ => {
                prog_option.file_path = args[idx].to_string();
            }
        }
        idx += 1;
    }
    return Some(prog_option);
}
