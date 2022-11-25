use std::{env, path::PathBuf};

use bytehound_res_to_csv::{get_data, write_data_to_stdout};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = PathBuf::from(args[0].clone());
    let data = get_data(&input_file);

    write_data_to_stdout(data);
}
