use std::{env, path::PathBuf};

use bytehound_res_to_csv::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    eprintln!("Bytehound result(.dat) to csv format");
    eprintln!(" - format: time(us), size");
    eprintln!(" - bin path: {}", args[0]);
    eprintln!(" - input bytehound dat: {}", args[1]);

    let input_file = PathBuf::from(args[1].clone());
    process(&input_file);
}
