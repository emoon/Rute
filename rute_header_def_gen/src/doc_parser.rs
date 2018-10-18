use pest::Parser;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use pest::iterators::Pair;
use std::io::{BufRead, BufReader};
use walkdir;
use walkdir::WalkDir;
use std::path::PathBuf;
use rayon::prelude::*;

pub struct DocParser;

#[derive(Debug, Clone)]
pub struct DocEntry {
    /// Which function this is attached to
    pub target_function: String,
    pub class_name: String,
    pub property: String,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct DocInfo {
    pub entries: Vec<DocEntry>,
    pub filename: String,
    pub base_filename: String,
}

///
///
///
impl DocEntry {
    fn new() -> DocEntry {
        DocEntry {
            target_function: String::new(),
            class_name: String::new(),
            property: String::new(),
            tags: Vec::new(),
        }
    }
}

#[derive(PartialEq)]
enum ParsingState {
    Doc,
    Global,
    SkipBlock,
}

fn is_header_file(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".cpp"))
        .unwrap_or(false)
}

fn is_private_file(entry: &walkdir::DirEntry) -> bool {
    entry.path().to_str().unwrap().contains("private")
}

///
/// Function for adding files/paths to process
///
fn add_process_path(dest: &mut Vec<PathBuf>, path: &str) {
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();

        if !is_header_file(&entry) {
            continue;
        }

        if is_private_file(&entry) {
            continue;
        }

        dest.push(entry.path().to_owned());
    }
}

///
/// Get class/property setting
///
fn get_class_prop(search_str: &String, search_tag: &'static str) -> String {
	if search_str.contains(search_tag) {
		let temp = search_str.split(" ");
		let mut return_next = false;

		for t in temp {
			if return_next {
				return t.to_owned();
			}

			if t == search_tag {
				return_next = true;
			}
		}
	}

	String::new()
}

impl DocParser {
    pub fn parse_files(paths: &[&str]) -> Vec<DocInfo> {
        let mut files: Vec<PathBuf> = Vec::new();

        for path in paths {
            add_process_path(&mut files, path);
        }

        // Process all files in parallel using Rayon
        let docs: Vec<DocInfo> = files.par_iter().map(|filename| {
            println!("Doc parsing {:?}", filename);
            Self::parse_file(filename)
        }).collect();

        docs
    }

    ///
    /// Parse a file
    ///
    pub fn parse_file<P: AsRef<Path>>(path: P) -> DocInfo {
        let mut buffer = String::new();

        let pathname = path.as_ref().to_str().unwrap();
        let buffer = BufReader::new(File::open(pathname).unwrap());

        let base_filename = Path::new(pathname).file_name().unwrap().to_str().unwrap();
        let base_filename = &base_filename[..base_filename.len() - 4];


        let mut docs = DocInfo {
            entries: Vec::new(),
            filename: pathname.to_owned(),
            base_filename: base_filename.to_owned(),
        };

        let mut current_doc: Option<DocEntry> = None;
        let mut parsing_state = ParsingState::Global;
        let mut block_indent_count = 0;

        for line in buffer.lines() {
            let current_line = line.unwrap();

            match current_line.as_str() {
                // start of doc
                "/*!" => {
                    if let Some(ref mut doc) = current_doc {
                        docs.entries.push(doc.clone());
                    }

                    current_doc = Some(DocEntry::new());
                    parsing_state = ParsingState::Doc;
                }

                // end of doc
                "*/" => {
                    parsing_state = ParsingState::Global;
                }

                // Skip CPP blocks
                "{" => {
                    if parsing_state == ParsingState::Global {
                        parsing_state = ParsingState::SkipBlock;
                    }

                    block_indent_count += 1;
                }

                // Skip CPP blocks
                "}" => {
                    block_indent_count -= 1;

                    if parsing_state == ParsingState::SkipBlock && block_indent_count == 0 {
                        parsing_state = ParsingState::Global;
                    }
                }

                _ => (),
            }

            if parsing_state == ParsingState::Doc {
				if let Some(ref mut doc) = current_doc {
					if doc.class_name.is_empty() {
						doc.class_name = get_class_prop(&current_line, "\\class");
					}

					if doc.property.is_empty() {
						doc.property = get_class_prop(&current_line, "\\property");
					}
				}
            }

            // Find CPP entry. This is fairly crude but should hopefully be find
            // If the line contains ::,(,) and we aren't in doc parsing mode we
            // will assume this is a function name

            if parsing_state == ParsingState::Global {
                if current_line.contains("::") &&
                   current_line.contains("(") &&
                   current_line.contains(")")
                {
                    let end_pos = current_line.find("(").unwrap();
                    let mut start_pos = 0;

                    // search backwards to find where the name starts
                    for (index, c) in current_line[..start_pos].chars().rev().enumerate() {
                        if c == '*' || c == ' ' || c == ' ' {
                            start_pos = index;
                            break;
                        }
                    }

                    if let Some(ref mut doc) = current_doc {
                    	if doc.target_function.is_empty() {
                        	doc.target_function = current_line[start_pos..end_pos].to_owned();
                    	}
                    }
                }
            } else {
                if !current_line.starts_with("/*!") && !current_line.starts_with("!*/") {
                    if let Some(ref mut doc) = current_doc {
                        if parsing_state == ParsingState::Doc {
                            doc.tags.push(current_line.clone());
                        }
                    }
                }
            }
        }

        if let Some(ref mut doc) = current_doc {
            docs.entries.push(doc.clone());
        }

        //for entry in &docs.entries {
            //for tag in &entry.tags {
            //   println!("{:?}", tag);
            //}
        //}

        docs
    }
}
