use std::env::args;

use crate::summary::{walk_dir, write_summary};

mod summary;

fn main() {
    let mut use_header: bool = true;

    let mut path = args().nth(1).expect("No path provided");
    if let Some(option) = args().nth(2){
        if option == "--filenames"{
            dbg!("read header");
            use_header = false;
        }
    }

    if !path.ends_with("/") {
        path.push_str("/");
    }
    println!("Autogenerated: {}SUMMARY.md", path);
    let entries = walk_dir(path.clone(), 0, use_header);
    write_summary(entries, &path);
}
