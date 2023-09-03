mod runtime;
use runtime::Runtime;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let runtime: Runtime = Runtime::initialize(args[1].clone());
}
