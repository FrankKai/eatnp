extern crate fs_extra;

use fs_extra::dir::get_size;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1].clone();
    run(dir);
}

fn run(dir: &String) {
    let reg = "node_modules".to_string();
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let opath = path.unwrap().path().into_os_string().into_string().unwrap();

        if Path::new(&opath).is_dir() {
            // delete current directory's node_modules
            if opath.ends_with(&reg) {
                let cpath = opath.clone();
                let dir_size = get_size(&cpath).unwrap() / 1024 / 1024;
                // ignore result
                fs::remove_dir_all(opath).ok();
                println!(
                    "deleted: {:80}| size {:?} MB",
                    &cpath,
                    dir_size,
                );
            } else {
                run(&opath);
            }
        }
    }
}
