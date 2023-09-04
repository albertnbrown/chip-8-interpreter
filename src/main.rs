mod runtime;
use runtime::{Runtime,Mode};
use std::env;
use std::process::exit;
use ctrlc;


fn main() {
    // despite alleged default handling for SIGINT, I needed this for it to work
    ctrlc::set_handler(move || {
        exit(1);
    }).expect("Error setting Ctrl-C handler");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mode: Mode;
    match args[2].as_str() {
        "0" => mode = Mode::CHIP8,
        "1" => mode = Mode::SCHIP,
        "2" => mode = Mode::X0CHIP,
        _ => panic!("bad compatability mode selected: {}", args[2]),
    }
    let mut runtime: Runtime = Runtime::initialize(args[1].clone(), mode);
    loop {
        runtime.frame();
    }
}
