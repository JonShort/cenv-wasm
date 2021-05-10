use regex::Regex;
use wasm_bindgen::prelude::*;
mod utils;
use utils::ParseStatus;
#[macro_use(lazy_static)]
extern crate lazy_static;

lazy_static! {
    static ref KEYWORD_REGEX: Regex = Regex::new(r"^#+ *\+\+ *(\w+)").unwrap();
}

pub fn parse_as_active(line: &str) -> String {
    let mut line_chars = line.chars();
    match line_chars.next() {
        Some('#') => {
            line_chars.next();
            String::from(line_chars.as_str())
        }
        Some(_) => String::from(line),
        None => String::from(line),
    }
}

pub fn parse_as_inactive(line: &str) -> String {
    let mut line_chars = line.chars();
    match line_chars.next() {
        Some('#') => String::from(line),
        Some(_) => format!("{}{}", "# ", line),
        None => String::from(line),
    }
}

pub fn resolve_keyword(line: &str) -> Option<String> {
    let keyword = match KEYWORD_REGEX.captures(line) {
        Some(caps) => caps.get(1).map_or("", |m| m.as_str()),
        None => return None,
    };

    Some(String::from(keyword))
}

#[wasm_bindgen]
pub fn parse_env(contents: String, keyword: String) -> String {
    let lines = contents.lines();

    let mut parse_status = ParseStatus::Ignore;

    let collected = lines.map(|line| {
        if line.is_empty() {
            parse_status = ParseStatus::Ignore;
            return String::from(line);
        }

        if let Some(line_keyword) = resolve_keyword(line) {
            if line_keyword == keyword {
                parse_status = ParseStatus::Active;
                return String::from(line);
            } else {
                parse_status = ParseStatus::Inactive;
                return String::from(line);
            }
        };

        match parse_status {
            ParseStatus::Active => parse_as_active(line),
            ParseStatus::Inactive => parse_as_inactive(line),
            ParseStatus::Ignore => String::from(line),
        }
    });

    let collected: Vec<String> = collected.collect();
    let collected = collected.join("\n");
    // Ensure we have an end-of-file newline
    let collected = collected + "\n";

    collected
}
