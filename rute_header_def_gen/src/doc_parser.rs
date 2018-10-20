use pest::Parser;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use pest::iterators::Pair;
use pest;
use std::fmt;
use std::fmt::Write;

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("qdoc.pest");

#[derive(Parser)]
#[grammar = "qdoc.pest"]
pub struct DocParser;

#[derive(Debug, Clone, Default)]
pub struct DocEntry {
    /// Which function this is attached to
    pub target_function: String,
    pub class_name: String,
    pub property: String,
    pub tags: String,
}

#[derive(Debug, Default)]
pub struct DocInfo {
    pub entries: Vec<DocEntry>,
    pub filename: String,
}

fn get_string(rule: Pair<Rule>) -> String {
	let mut dest = rule.as_str().to_owned(); 
	if !dest.starts_with(',') &&
	   !dest.starts_with('\'') &&
	   !dest.starts_with('&') 
	{
		dest.push(' ');
	}

	dest
}

///
///
fn build_strings(rule: Pair<Rule>) -> String {
	let mut temp = String::new();

    for entry in rule.into_inner() {
        match entry.as_rule() {
            Rule::name => {
				temp.push_str(&get_string(entry));
            }
            _ => (),
        }
    }

	temp
}

fn get_block_string(rule: Pair<Rule>) -> String {
    for entry in rule.into_inner() {
        match entry.as_rule() {
            Rule::block_string => return build_strings(entry), 
            _ => (),
        }
    }

    String::new()
}

fn get_name_list(rule: Pair<Rule>, doc_entry: &mut DocEntry) -> String {
    for entry in rule.into_inner() {
        match entry.as_rule() {
            Rule::namelist => return build_strings(entry), 
            Rule::tags => process_tags(entry, doc_entry),
            _ => (),
        }
    }

    String::new()
}


///
///
///
fn process_tags(rule: Pair<Rule>, doc_entry: &mut DocEntry) {
    for entry in rule.into_inner() {
		match entry.as_rule() {
			// Rule::abstract_tag => (), 
			// Rule::bad_code_tag => (), 
			Rule::bold_tag => {
				doc_entry.tags.push_str(&format!(" **{}** ", get_block_string(entry)));
			}

			Rule::brief_tag => {
				//println!("{:#?}", entry);
				let tag = get_name_list(entry, doc_entry);
				doc_entry.tags.push_str(&format!("{}\n", tag)); 
			}

			// Rule::c_tag => (), 
			// Rule::caption_tag => (), 
			// Rule::class_tag => (), 
			// Rule::code_tag => (), 
			// Rule::codeline_tag => (), 
			// Rule::compat_tag => (), 
			// Rule::contentspage_tag => (), 
			// Rule::div_tag => (), 
			// Rule::dots_tag => (), 
			// Rule::else_tag => (), 
			// Rule::emphasis_tag => (), 
			// Rule::endif_tag => (), 
			// Rule::endlist_tag => (), 
			// Rule::enum_tag => (), 
			// Rule::example_tag => (), 
			// Rule::externalpage_tag => (), 
			// Rule::footnote_tag => (), 
			// Rule::function_tag => (), 
			// Rule::generatelist_tag => (), 
			// Rule::group_tag => (), 
			// Rule::header_tag => (), 
			// Rule::headerfile_tag => (), 
			// Rule::i_tag => (), 
			// Rule::if_tag => (), 
			// Rule::image_tag => (), 
			// Rule::include_tag => (), 
			// Rule::indexpage_tag => (), 
			// Rule::ingroup_tag => (), 
			// Rule::inherits_tag => (), 
			// Rule::inlineimage_tag => (), 
			// Rule::inmodule_tag => (), 
			// Rule::inqmlmodule_tag => (), 
			// Rule::instantiates_tag => (), 
			// Rule::internal_tag => (), 
			// Rule::keyword_tag => (), 
			// Rule::l_tag => (), 
			// Rule::legalese_tag => (), 
			// Rule::li_tag => (), 
			// Rule::list_tag => (), 
			// Rule::macro_tag => (), 
			// Rule::meta_tag => (), 
			// Rule::module_tag => (), 
			// Rule::namespace_tag => (), 
			// Rule::newcode_tag => (), 
			// Rule::nextpage_tag => (), 
			// Rule::noautolist_tag => (), 
			// Rule::nonreentrant_tag => (), 
			// Rule::note_tag => (), 
			// Rule::o_tag => (), 
			// Rule::obsolete_tag => (), 
			// Rule::oldcode_tag => (), 
			// Rule::omit_tag => (), 
			// Rule::omitvalue_tag => (), 
			// Rule::overload_tag => (), 
			// Rule::page_tag => (), 
			// Rule::parameter_tag => (), 
			// Rule::preliminary_tag => (), 
			// Rule::previouspage_tag => (), 
			// Rule::printline_tag => (), 
			// Rule::printto_tag => (), 
			// Rule::printuntil_tag => (), 
			Rule::property_tag => doc_entry.property = get_cpp_name(entry), 
			// Rule::quotation_tag => (), 
			// Rule::quotefile_tag => (), 
			// Rule::quotefromfile_tag => (), 
			// Rule::raw_tag => (), 
			// Rule::reentrant_tag => (), 
			// Rule::reimp_tag => (), 
			// Rule::relates_tag => (), 
			// Rule::row_tag => (), 
			// Rule::sa_tag => (), 
			// Rule::section1_tag => (), 
			// Rule::section2_tag => (), 
			// Rule::section3_tag => (), 
			// Rule::section4_tag => (), 
			// Rule::since_tag => (), 
			// Rule::skipline_tag => (), 
			// Rule::skipto_tag => (), 
			// Rule::skipuntil_tag => (), 
			// Rule::snippet_tag => (), 
			// Rule::span_tag => (), 
			// Rule::startpage_tag => (), 
			// Rule::sub_tag => (), 
			// Rule::subtitle_tag => (), 
			Rule::tags => process_tags(entry, doc_entry), 
			//Rule::sup_tag => (), 
			// Rule::table_tag => (), 
			// Rule::tableofcontents_tag => (), 
			// Rule::target_tag => (), 
			// Rule::threadsafe_tag => (), 
			// Rule::title_tag => (), 
			// Rule::tt_tag => (), 
			// Rule::typedef_tag => (), 
			// Rule::uicontrol_tag => (), 
			// Rule::underline_tag => (), 
			// Rule::value_tag => (), 
			// Rule::variable_tag => (), 
			// Rule::warning_tag => (), 
			Rule::name => doc_entry.tags.push_str(&get_string(entry)),
			_ => println!("tag {:?} not tracked!", entry.as_rule()),
		}
	}
}

///
///
///
fn get_doc(rule: Pair<Rule>) -> DocEntry {
    let mut doc_entry = DocEntry::default();

    process_tags(rule, &mut doc_entry);

    //println!("---------------------------------------------------------------------------");

    println!("{:#?}", doc_entry);

    doc_entry
}

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

        let mut docs = DocInfo::default();
        //let mut current_doc = None;

        //println!("{:#?}", chunks);

        for chunk in chunks {
            match chunk.as_rule() {
                Rule::doc_start => {
                    let current_doc = Some(get_doc(chunk));
                }

                Rule::skip_line => {
                    println!("skip_line {}", chunk.as_str());
                }

                Rule::cpp_func_def => {
					//println!("cpp_name {}", get_cpp_name(chunk));
                }

                _ => (),
            }
        }

        //println!("{:?}", docs);
    }

}
