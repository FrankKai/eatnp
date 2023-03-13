use std::env;
use std::process;

use eatnp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1].clone();
    if let Err(e) = eatnp::run(dir) {
        println!("eatnp run error: {e}");
        process::exit(1);
    }
}
