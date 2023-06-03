use std::{path::PathBuf, fs::File, io::{self, Write}, fmt::format};

use cli_core::{
    parse_events,
    Data, Loader, Allocation, Timestamp, Operation,
    event::Event,
};

pub fn process(path: &PathBuf) {
    eprintln!(" - Get event stream");
    let file = File::open(path).unwrap();
    let (header, event_stream) = parse_events(file).unwrap();
    
    eprintln!(" - Process events");
    for event in event_stream {
        let event = event.unwrap();
        process_event(event)
    }
}

pub fn process_event(event: Event) {
    //
}

pub fn get_data(path: &PathBuf) -> Data {
    let file = File::open(path).unwrap();
    let debug_symbols = vec![path.clone()];
    let data = Loader::load_from_stream( file, debug_symbols ).unwrap();

    data
}

pub fn get_data(path: &PathBuf) -> Data {
    let file = File::open(path).unwrap();
    let debug_symbols = vec![path.clone()];
    let data = Loader::load_from_stream( file, debug_symbols ).unwrap();

    data
}

pub fn write_data_to_stdout(data: Data) {
    let mut stdout = io::stdout();

    let init_time = data.initial_timestamp();

    for op in data.operations() {
        match op {
            Operation::Allocation { allocation, .. } => {
                let size = size_of_allocation(&allocation);
                let td = time_delta_of_allocation(&allocation, &init_time);
                let line = format!("{},{}\n", td, size);
                stdout.write(line.as_bytes()).unwrap();
            },
            Operation::Deallocation { allocation, .. } => {
                let size = size_of_allocation(&allocation);
                let td = time_delta_of_deallocation(&allocation, &init_time);
                let line = format!("{},-{}\n", td, size);
                stdout.write(line.as_bytes()).unwrap();
            },
            Operation::Reallocation { old_allocation, new_allocation, .. } => {
                {
                    let size = size_of_allocation(&old_allocation);
                    let td = time_delta_of_deallocation(&old_allocation, &init_time);
                    let line = format!("{},-{}\n", td, size);
                    stdout.write(line.as_bytes()).unwrap();
                }
                {
                    let size = size_of_allocation(&new_allocation);
                    let td = time_delta_of_allocation(&new_allocation, &init_time);
                    let line = format!("{},{}\n", td, size);
                    stdout.write(line.as_bytes()).unwrap();
                }
            }
        }
    }
}

fn size_of_allocation(allocation: &Allocation) -> u64 {
    allocation.size + allocation.extra_usable_space as u64
}
fn time_delta_of_allocation(allocation: &Allocation, init_timestamp: &Timestamp) -> u64 {
    let d = allocation.timestamp - init_timestamp.clone();
    us_of_timestamp(d)
}
fn time_delta_of_deallocation(allocation: &Allocation, init_timestamp: &Timestamp) -> u64 {
    let deallocation = allocation.deallocation.as_ref().unwrap();
    let d = deallocation.timestamp.clone() - init_timestamp.clone();
    us_of_timestamp(d)
}
fn us_of_timestamp(timestamp: Timestamp) -> u64 {
    timestamp.as_usecs()
}
