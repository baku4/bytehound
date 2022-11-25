use std::{path::PathBuf, fs::File, io::{self, Write}, fmt::format};

use cli_core::{Data, Loader, Allocation, Timestamp, Operation};

pub fn get_data(path: &PathBuf) -> Data {
    let file = File::open(path).unwrap();
    let debug_symbols = vec![path.clone()];
    let data = Loader::load_from_stream( file, debug_symbols ).unwrap();

    data
}

pub fn write_data_to_stdout(data: Data) {
    let mut stdout = io::stdout();

    let init_time = us_of_timestamp(data.initial_timestamp());

    for op in data.operations() {
        match op {
            Operation::Allocation { allocation, .. } => {
                let size = size_of_allocation(&allocation);
                let td = time_delta_of_allocation(&allocation, init_time);
                let line = format!("{},{}\n", td, size);
                stdout.write(line.as_bytes()).unwrap();
            },
            Operation::Deallocation { allocation, .. } => {
                let size = size_of_allocation(&allocation);
                let td = time_delta_of_allocation(&allocation, init_time);
                let line = format!("{},-{}\n", td, size);
                stdout.write(line.as_bytes()).unwrap();
            },
            Operation::Reallocation { old_allocation, new_allocation, .. } => {
                {
                    let size = size_of_allocation(&old_allocation);
                    let td = time_delta_of_allocation(&old_allocation, init_time);
                    let line = format!("{},-{}\n", td, size);
                    stdout.write(line.as_bytes()).unwrap();
                }
                {
                    let size = size_of_allocation(&new_allocation);
                    let td = time_delta_of_allocation(&new_allocation, init_time);
                    let line = format!("{},{}\n", td, size);
                    stdout.write(line.as_bytes()).unwrap();
                }
            }
        }
    }
}

fn us_of_timestamp(timestamp: Timestamp) -> u64 {
    timestamp.as_usecs()
}

fn size_of_allocation(allocation: &Allocation) -> u64 {
    allocation.size + allocation.extra_usable_space as u64
}
fn time_delta_of_allocation(allocation: &Allocation, time: u64) -> u64 {
    us_of_timestamp(allocation.timestamp) - time
}
