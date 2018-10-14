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


        for chunk in chunks {
            match chunk.as_rule() {
                Rule::doc_start => {
                    println!("    {:?}", chunk.as_str());
                }

                Rule::skip_line => {
                    //println!("    {:?}", chunk.as_str());
                }

                Rule::cpp_func_def => {
                    println!("    {:?}", chunk.as_str());
                }

                _ => (),
            }
        }
        //println!("{:?}", chunks);
    }
}
