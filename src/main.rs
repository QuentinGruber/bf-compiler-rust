use std::env;
use std::fs;
mod utils;
use utils::*;
mod runtime;
use runtime::BfRuntime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let file = fs::read_to_string(get_full_file_path(file_path))
        .expect("Something went wrong reading the file");

    let mut rt = BfRuntime {
        memory: [0; 30000],
        cursor: 0,
        loop_cursor: 0,
        is_in_loop: false,
        loop_heap: "".to_owned(),
    };
    rt.process_bf(file);
}
