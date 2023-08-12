use std::{
    fs::{read_dir, read_to_string, DirEntry, OpenOptions},
    io::Write, iter::Peekable, vec::IntoIter,
};

#[derive(Debug, Clone)]
pub enum Entry {
    MarkdownFile {
        title: String,
        path: String,
        depth: usize,
    },
    MarkdownFolder {
        title: String,
        path: Option<String>,
        depth: usize,
    },
}

const SUMMARY_MD: &str = "SUMMARY.md";
const README_MD: &str = "README.md";

pub fn write_summary(entries: Vec<Entry>, root_path: &str) {
    let mut lines: Vec<String> = vec!["# Summary\n\n[Welcome](README.md)\n\n----".into()];
    for entry in entries {
        match entry {
            Entry::MarkdownFolder { title, path, depth } => {
                let whitespace = "\t".repeat(depth);
                let link = path
                    .unwrap_or_default()
                    .trim_start_matches(&root_path)
                    .to_string();
                lines.push(format!("{}* [{}]({})", whitespace, title, link));
            }
            Entry::MarkdownFile { title, path, depth } => {
                let whitespace = "\t".repeat(depth);
                lines.push(format!(
                    "{}* [{}]({})",
                    whitespace,
                    title,
                    path.trim_start_matches(&root_path).to_string()
                ));
            }
        }
    }
    let str = lines.join("\n");
    write_file(str, root_path);
}

fn write_file(file_str: String, root_path: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(root_path.to_string() + SUMMARY_MD)
        .unwrap();
    file.write_all(file_str.as_bytes()).unwrap();
}

pub fn walk_dir(dir: String, depth: usize, use_header: bool) -> Vec<Entry> {
    let mut result = vec![];
    let mut entries: Vec<DirEntry> = read_dir(&dir).unwrap().map(|r| r.unwrap()).collect();
    entries.sort_by_key(|e| e.path().to_str().unwrap().to_lowercase());
    let mut entries: Peekable<IntoIter<DirEntry>> =
        entries.into_iter().peekable();

    while let Some(entry) = entries.next() {
        let entry = entry;

        if entry.file_type().unwrap().is_dir() {
            let current = Entry::MarkdownFolder {
                title: get_title(&entry, use_header),
                depth,
                path: None,
            };
            let mut inner_entries = walk_dir(
                entry.path().to_str().unwrap().to_string(),
                depth + 1,
                use_header,
            );
            // check if 'foldername.md' exists for current 'foldername':
            if let Some(next_entry) = entries.peek() {
                if next_entry
                    .file_name()
                    .into_string()
                    .unwrap()
                    .trim_end_matches(".md")
                    .to_string()
                    == entry.file_name().into_string().unwrap()
                {
                    result.push(Entry::MarkdownFolder {
                        title: get_title(&next_entry, use_header),
                        depth,
                        path: Some(next_entry.path().into_os_string().into_string().unwrap()),
                    });
                    result.append(&mut inner_entries);
                    continue;
                }
            }
            result.push(current);
            result.append(&mut inner_entries);
            continue;
        }

        let path = entry.path().as_os_str().to_str().unwrap().to_string();
        if path.ends_with(SUMMARY_MD) {
            continue;
        }
        if depth == 0 && path.ends_with(README_MD) {
            continue;
        }

        let extension = entry
            .path()
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if extension == "md" {
            result.push(Entry::MarkdownFile {
                title: get_title(&entry, use_header),
                path,
                depth,
            });
        }
    }
    return result;
}


/// Reads the first H1 header in a markdown file to get the title of the chapter
fn get_title(entry: &DirEntry, use_header: bool) -> String {
    fn get_dir_as_title(entry: &DirEntry) -> String {
        return entry
            .file_name()
            .to_str()
            .unwrap()
            .trim_end_matches(".md")
            .to_string();
    }
    if !use_header || entry.file_type().unwrap().is_dir() {
        return get_dir_as_title(entry);
    }
    let contents = read_to_string(entry.path()).unwrap();
    let mut title = None;
    for mut l in contents.lines() {
        if l.starts_with("\u{feff}"){
           l = l.trim_start_matches("\u{feff}"); 
        }
        if l.starts_with("# ") {
            if let Some(h1) = l.get(2..) {
                title = Some(String::from(h1))
            }
            break;
        }
    }
    title.unwrap_or(get_dir_as_title(entry))
}
