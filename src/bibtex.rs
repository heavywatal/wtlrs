use std::collections::HashSet;
use regex::Regex;

pub fn filter(content: &str, citekeys: &HashSet::<String>) -> String {
    let fields = HashSet::from(["author", "title", "journal", "year",
        "volume", "number", "pages", "publisher", "address", "editor"]);
    let re = Regex::new(r"(?ms)@\w+?\{(\S+?),.+?\}\n").unwrap();
    let mut citekeys = citekeys.clone();
    let mut s = String::new();
    for caps in re.captures_iter(content) {
        if citekeys.remove(&caps[1]) {
            s.push_str(&select(&caps[0], &fields));
            s.push('\n');
            if citekeys.is_empty() {break;}
        }
    }
    return s;
}

fn select(entry: &str, fields: &HashSet::<&str>) -> String {
    let mut s = String::new();
    let mut lines = entry.split_inclusive('\n');
    s.push_str(lines.next().unwrap()); // @article
    for line in lines {
        if fields.contains(line.split(" = ").next().unwrap().trim_start()) {
            s.push_str(line);
        }
    }
    s.replace_range((s.len() - 2).., "}\n");
    return s;
}
