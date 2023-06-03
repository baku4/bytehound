use std::{
    path::PathBuf,
    fs::File,
    io::{
        Write,
    },
};
use std::{
    io::{BufWriter, StdoutLock},
};
use cli_core::{
    parse_events,
    Data, Loader, Timestamp,
    event::{Event, AllocBody},
};

use ahash::AHashMap;

pub fn process(path: &PathBuf) {
    eprintln!(" - Init buffers");
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut buf_writer = std::io::BufWriter::with_capacity(
        32 * 1024,
        lock,
    );
    let mut size_map = SizeMap::new();

    eprintln!(" - Get event stream");
    let file = File::open(path).unwrap();
    let (header, event_stream) = parse_events(file).unwrap();
    let initial_time_stamp = header.initial_timestamp;

    eprintln!(" - Process events");
    for event in event_stream {
        let event = event.unwrap();
        process_event(
            initial_time_stamp,
            event,
            &mut size_map,
            &mut buf_writer,
        );
    }
}

struct SizeMap {
    map: AHashMap<u64, u64>
}
impl SizeMap {
    fn new() -> Self {
        Self {
            map: AHashMap::new(),
        }
    }
    fn add_new_allocation(
        &mut self,
        alloc_id: u64,
        size: u64,
    ) {
        self.map.insert(alloc_id, size);
    }
    fn get_previous_alloc_size(
        &mut self,
        alloc_id: u64,
    ) -> Option<u64> {
        self.map.remove(&alloc_id)
    }
}

fn process_event(
    initial_time_stamp: Timestamp,
    event: Event,
    size_map: &mut SizeMap,
    buf_writer: &mut BufWriter<StdoutLock>,
) {
    match event {
        Event::Alloc { timestamp, allocation, } => {
            print_alloc_line(timestamp-initial_time_stamp, allocation, size_map, buf_writer);
        },
        Event::AllocEx { id: _, timestamp, allocation } => {
            print_alloc_line(timestamp-initial_time_stamp, allocation, size_map, buf_writer);
        },
        Event::Realloc { timestamp, old_pointer, allocation } => {
            print_realloc_line(timestamp-initial_time_stamp, old_pointer, allocation, size_map, buf_writer);
        },
        Event::ReallocEx { id: _, timestamp, old_pointer, allocation } => {
            print_realloc_line(timestamp-initial_time_stamp, old_pointer, allocation, size_map, buf_writer);
        },
        Event::Free { timestamp, pointer, backtrace: _, thread: _ } => {
            print_free_line(timestamp-initial_time_stamp, pointer, size_map, buf_writer);
        },
        Event::FreeEx { id: _, timestamp, pointer, backtrace: _, thread: _ } => {
            print_free_line(timestamp-initial_time_stamp, pointer, size_map, buf_writer);
        },
        _ => {
            // pass
        }
    }
}

pub fn get_data(path: &PathBuf) -> Data {
    let file = File::open(path).unwrap();
    let debug_symbols = vec![path.clone()];
    let data = Loader::load_from_stream( file, debug_symbols ).unwrap();

    data
}

fn print_alloc_line(
    timestamp: Timestamp,
    allocation: AllocBody,
    size_map: &mut SizeMap,
    buf_writer: &mut BufWriter<StdoutLock>,
) {
    let time = us_of_timestamp(timestamp);
    let size = allocation.size + allocation.extra_usable_space as u64;
    let alloc_id = allocation.pointer;

    size_map.add_new_allocation(alloc_id, size);
    let line = format!("{},{}\n", time, size);
    let _ = buf_writer.write(line.as_bytes()).unwrap();
}
fn print_realloc_line(
    timestamp: Timestamp,
    previos_alloc_id: u64,
    allocation: AllocBody,
    size_map: &mut SizeMap,
    buf_writer: &mut BufWriter<StdoutLock>,
) {
    print_free_line(timestamp, previos_alloc_id, size_map, buf_writer);
    print_alloc_line(timestamp, allocation, size_map, buf_writer);
}
fn print_free_line(
    timestamp: Timestamp,
    previos_alloc_id: u64,
    size_map: &mut SizeMap,
    buf_writer: &mut BufWriter<StdoutLock>,
) {
    let previous_alloc_size = size_map.get_previous_alloc_size(previos_alloc_id);
    if let Some(size) = previous_alloc_size {
        let time = us_of_timestamp(timestamp);
        let line = format!("{},-{}\n", time, size);
        let _ = buf_writer.write(line.as_bytes()).unwrap();
    }
}

fn us_of_timestamp(timestamp: Timestamp) -> u64 {
    timestamp.as_usecs()
}
