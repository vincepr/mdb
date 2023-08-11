use std::{fs::{self, DirEntry}, io::{prelude::*,Error, Read, BufReader, BufWriter}, path::Path, fmt, ffi::OsStr};



// #[derive(Debug, Clone)]
// pub struct MarkdownFile {
//     pub title: String,
//     pub path: String,
//     pub link: String,
// }


// #[derive(Debug, Clone)]
// pub struct MarkdownFolder {
//     pub title: String,
//     pub path: String,
//     /// subfolders/
//     pub entries: Vec<Entry>,
// }

#[derive(Debug, Clone)]
pub enum Entry{
    MarkdownFile{
        title: String,
        path: String,
        link: String,
        depth: usize,
    },
    MarkdownFolder,
}

fn main() {
    let path = std::env::args().nth(1).expect("No path given");
    println!("Starting from folder: {}", path);
    let entries = walk_dir(path.clone());
    write_summary(entries, &path);
}

fn write_summary(entries: Vec<Entry>, path : &str, depth: usize) {
    let mut lines : Vec<String> = vec![];
    for entry in entries.clone() {
        if let Entry::MarkdownFile{title, path:_, link, depth} = entry {
            let whitespace = dep
            lines.push(format!("* [{}]({})", title, link));
        }
    }
    let buff: String = lines.join("\n");

    let summary_file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path.to_string() + "/" + SUMMARY_FILE)
        .unwrap();

    let mut old_summary_file_content = String::new();
    let mut summary_file_reader = BufReader::new(summary_file);
    summary_file_reader.read_to_string(&mut old_summary_file_content).unwrap();

    //let old_md5_string = md5(&old_summary_file_content);

    // if new_md5_string == old_md5_string {
    //     return;
    // }

    let summary_file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(path.to_string() + "/" + SUMMARY_FILE)
        .unwrap();
    let mut summary_file_writer = BufWriter::new(summary_file);
    summary_file_writer.write_all(buff.as_bytes()).unwrap();
    
    
    
    

}

fn walk_dir(mut dir: String) -> Vec<Entry>{
    if !dir.ends_with("/") {
        dir.push_str("/");
    }

    let mut result = vec![];
    let mut entries: Vec<DirEntry> = fs::read_dir(&dir).unwrap().map(|r| r.unwrap()).collect();
    entries.sort_by_key(|e| e.path().to_str().unwrap().to_lowercase());

    // loop over alphanumerical sorted files and folders:
    for entry in entries {
        let entry = entry;
        if entry.file_type().unwrap().is_dir(){
            let mut inner_entries = walk_dir(entry.path().to_str().unwrap().to_string());
            result.append(&mut inner_entries);
            continue;
        }
        let path = entry.path().as_os_str().to_str().unwrap().to_string();
        let extension = entry.path().extension().unwrap().to_str().unwrap().to_string();
        if extension == "md" {
            result.push(Entry::MarkdownFile{ 
                title: get_title(entry), 
                link: path.trim_start_matches(&dir).to_string(),
                path,
            });
        }
    }
    return result;
}

fn get_title(entry: DirEntry) -> String {
    return entry.file_name().to_str().unwrap().to_string()
}
// fn walk_dir_old(dir: String) -> Result<MarkdownFolder, Error>{
//     let entries = fs::read_dir(&dir)?;
//     let mut root = MarkdownFolder{
//         name: Path::new(&dir).file_name().unwrap().to_str().unwrap().to_string(),
//         path: dir,
//         folders: vec![],
//         files: vec![],
//     };

//     for entry in entries {
//         let entry = entry.unwrap();
//         if entry.file_type()?.is_dir(){
//             let files = walk_dir(entry.path().to_str().unwrap().into())?;
//             root.folders.push(files);
//             continue;
//         }
//         let name = entry.file_name().to_str().unwrap().to_string();
//         let arr: Vec<&str> = name.split(".").collect();
//         if arr.len() < 2 {
//             continue;
//         }
//         let name = arr[0];
//         let extension = arr[1];
//         if extension.to_lowercase() != "md" {
//             continue;
//         }
//         let title = get_title(&entry);
//         let md = MarkdownFile{
//             name: name.into(),
//             path: entry.path().to_str().unwrap().to_string(),
//             title,
//         };
//         root.files.push(md);
//     }
//     Ok(root)
// }

const SUMMARY_FILE: &str = "SUMMARY.md";
const README_MD: &str = "README.md";
