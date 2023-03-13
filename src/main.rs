use std::env;

use eatnp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1].clone();
    eatnp::run(dir);
}
