use std::{env, path::PathBuf};

use bytehound_res_to_csv::{get_data, write_data_to_stdout};

fn main() {
    let args: Vec<String> = env::args().collect();
    eprintln!("Bytehound result(.dat) to csv format");
    eprintln!(" - format: time(ns), size");
    eprintln!(" - bin path: {}", args[0]);
    eprintln!(" - input bytehound dat: {}", args[1]);

    let input_file = PathBuf::from(args[1].clone());
    let data = get_data(&input_file);

    write_data_to_stdout(data);
}
