use glob::glob;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1].clone();
    let reg = "/node_modules/".to_string();
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        // println!("Name: {}", path.unwrap().path().display());
        let opath = path.unwrap().path().into_os_string().into_string().unwrap();
        // println!("Bol: {}", Path::new(&opath).is_dir());
        if Path::new(&opath).is_dir() {
            let vpath = opath + &reg;
            // println!("Path: {}", &opath);
            for entry in glob(&vpath).expect("Failed to read glob pattern") {
                match entry {
                    Ok(apath) => {
                        println!("deleted: {:?}", apath.display());
                        fs::remove_dir_all(apath);
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    }
}