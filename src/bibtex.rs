//! Parse `.bib` files.

use once_cell::sync::OnceCell;
use regex::{Captures, Regex};
use std::collections::HashSet;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: OnceCell<Regex> = OnceCell::new();
        RE.get_or_init(|| Regex::new($re).unwrap())
    }};
}

/// Filter bib entries with citekeys.
pub fn filter(content: &str, citekeys: &HashSet<String>) -> String {
    let fields = HashSet::from([
        "author",
        "title",
        "journal",
        "year",
        "volume",
        "number",
        "pages",
        "publisher",
        "address",
        "editor",
    ]);
    let re = Regex::new(r"(?ms)@\w+?\{(\S+?),.+?\}\n").unwrap();
    let mut citekeys = citekeys.clone();
    let mut s = String::new();
    for caps in re.captures_iter(content) {
        if citekeys.remove(&caps[1]) {
            s.push_str(&select(&caps[0], &fields));
            s.push('\n');
            if citekeys.is_empty() {
                break;
            }
        }
    }
    s
}

fn select(entry: &str, fields: &HashSet<&str>) -> String {
    let mut s = String::new();
    let mut lines = entry.split_inclusive('\n');
    s.push_str(lines.next().unwrap()); // @article
    for line in lines {
        let field = line.split(" = ").next().unwrap().trim_start();
        if fields.contains(field) {
            if field == "pages" {
                s.push_str(&normalize_pages(line));
            } else {
                s.push_str(line);
            }
        }
    }
    s.replace_range((s.len() - 2).., "}\n");
    s
}

fn normalize_pages(line: &str) -> String {
    let re = regex!(r"^(\s*pages = )\{(\d+)-+(\d+)\}");
    let repl = |caps: &Captures| -> String {
        let start = caps[2].to_string();
        let mut end = caps[3].to_string();
        if start.len() > end.len() {
            let mut new_end = start.clone();
            new_end.replace_range((start.len() - end.len()).., &end);
            end = new_end;
        }
        return format!("{}{{{}--{}}}", &caps[1], start, end);
    };
    return re.replace(line, repl).to_string();
}
