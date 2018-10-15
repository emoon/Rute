use pest::Parser;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use pest::iterators::Pair;

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("qdoc.pest");

#[derive(Parser)]
#[grammar = "qdoc.pest"]
pub struct DocParser;

///
/// Supported tags from QDoc
///
#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Class(String),
    Brief(String),
    InGroup(String),
    ListStart(String),
    ListEntry(String),
    ListEnd(String),
    Image(String),
    Property(String),
    SaTag(String),
    /// Just text that doesn't belong to a tag
    GeneralText(String),
    UnTracked(String),
}

#[derive(Debug, Clone)]
pub struct DocEntry {
    /// Which function this is attached to
    target_function: Option<String>,
    tags: Vec<Tag>,
}

#[derive(Debug)]
pub struct DocInfo {
    entries: Vec<DocEntry>,
}

///
///
///
fn get_doc(rule: Pair<Rule>) -> DocEntry {
    let mut entry = DocEntry {
        target_function: None,
        tags: Vec::new(),
    };

    /*
    for entry in rule.into_inner() {

    }
    */

    entry
}

///
///
///
fn get_cpp_name(rule: Pair<Rule>) -> String {
    for entry in rule.into_inner() {
        match entry.as_rule() {
            Rule::cpp_name => return entry.as_str().to_owned(),
            _ => (),
        }
    }

    "".to_owned()
}

impl DocParser {
    ///
    /// Parse a file
    ///
    pub fn parse_file<P: AsRef<Path>>(path: P) {
        let mut buffer = String::new();

        let pathname = path.as_ref().to_str().unwrap();

        let mut f = File::open(pathname)
            .unwrap_or_else(|e| panic!("ApiParser: Unable to open {}: {}", pathname, e));
        f.read_to_string(&mut buffer)
            .unwrap_or_else(|e| panic!("ApiParser: Unable to read from {}: {}", pathname, e));

        Self::parse_string(&buffer, &pathname);
    }

    ///
    /// Parse a string
    ///
    pub fn parse_string(buffer: &str, filename: &str) {
        let chunks = Self::parse(Rule::chunk, buffer)
            .unwrap_or_else(|e| panic!("APiParser: {} {}", filename, e));

        let mut docs = DocInfo { entries: Vec::new() };
        let mut current_doc = None;

        for chunk in chunks {
            match chunk.as_rule() {
                Rule::doc_start => {
                    current_doc = Some(get_doc(chunk))
                }

                Rule::skip_line => {
                    //println!("    {:?}", chunk.as_str());
                }

                Rule::cpp_func_def => {
                    if let Some(ref mut doc) = current_doc {
                        doc.target_function = Some(get_cpp_name(chunk));
                        docs.entries.push(doc.clone());
                    }

                    current_doc = None;

                    //println!("    {:?}", chunk.as_str());
                }

                _ => (),
            }
        }
        //println!("{:?}", chunks);

        println!("{:?}", docs);
    }

}
