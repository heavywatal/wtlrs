use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};

pub fn resolve_ref(content: &str, labelmap: &HashMap<String, String>) -> String {
    let re = Regex::new(r"\\ref\{(.+?)\}").unwrap();
    let repl = |caps: &Captures| -> String { labelmap[&caps[1]].to_string() };
    return re.replace_all(content, repl).to_string();
}

pub fn remove_asterisk(content: &str) -> String {
    let re = Regex::new(r"\{(table|figure)\*\}").unwrap();
    return re.replace_all(content, "{$1}").to_string();
}

pub fn label_caption(content: &str, labelmap: &HashMap<String, String>) -> String {
    let re = Regex::new(r"(?s)caption\{(.+?)\\label\{([^}]+)\}").unwrap();
    let repl = |caps: &Captures| -> String {
        let num = &labelmap[&caps[2]];
        let label = classify_label(&caps[2]);
        let s = format!(r"caption{{\textbf{{{} {}}}. {}", label, num, &caps[1]);
        eprintln!("{}…", &s[..std::cmp::min(60, s.len())]);
        s
    };
    return re.replace_all(content, repl).to_string();
}

fn classify_label(label: &str) -> &str {
    let ll = label.to_lowercase();
    if ll.starts_with("fig") {
        "Figure"
    } else if ll.starts_with("tab") {
        "Table"
    } else {
        "Equation???"
    }
}

pub fn collect_citekeys(aux: &str) -> HashSet<String> {
    let mut set = HashSet::<String>::new();
    let re = Regex::new(r"\\citation\{(.+?)\}").unwrap();
    for caps in re.captures_iter(aux) {
        for k in caps[1].split(',') {
            set.insert(k.to_string());
        }
    }
    set
}

pub fn collect_labels(aux: &str) -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    let re = Regex::new(r"\\newlabel\{(.+?)\}\{\{(.+?)\}").unwrap();
    for caps in re.captures_iter(aux) {
        map.insert(caps[1].to_string(), caps[2].to_string());
    }
    map
}
