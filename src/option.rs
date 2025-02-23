use std::env;

pub struct ProgramOption {
    pub file_path: String,
    pub prefer_pop_color: usize,
    pub output_file: String,
}

impl ProgramOption {
    pub fn default_value() -> ProgramOption {
        return ProgramOption {
            file_path: String::new(),
            prefer_pop_color: 55,
            output_file: String::new(),
        };
    }
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
            _ => {
                prog_option.file_path = args[idx].to_string();
            }
        }
        idx += 1;
    }
    return Some(prog_option);
}
