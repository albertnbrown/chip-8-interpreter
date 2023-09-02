mod runtime;
use runtime::storage::Storage;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let storage: Storage = Storage::initialize(args[1].clone());
    storage.show_memory();
}
