//! Parse `.tex` and `.aux` files.

use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};

/// Resolve `\ref{}` with a label map extracted from `.aux` file.
pub fn resolve_ref(content: &str, labelmap: &HashMap<String, String>) -> String {
    let re = Regex::new(r"\\ref\{(.+?)\}").unwrap();
    let repl = |caps: &Captures| -> String { labelmap[&caps[1]].to_string() };
    return re.replace_all(content, repl).to_string();
}

/// Remove trailing asterisks from `{table*}` and `{figure*}`.
pub fn remove_asterisk(content: &str) -> String {
    let re = Regex::new(r"\{(table|figure)\*\}").unwrap();
    return re.replace_all(content, "{$1}").to_string();
}

/// Add a bold label (e.g., **Figure 1.**) to the top of each caption.
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

/// Extract citation keys from `.aux` content.
pub fn collect_citekeys(aux: &str) -> HashSet<String> {
    let mut set = HashSet::<String>::new();
    let re = Regex::new(r"\\(citation|abx@aux@cite\{0\})\{(.+?)\}").unwrap();
    for caps in re.captures_iter(aux) {
        for k in caps[2].split(',') {
            set.insert(k.to_string());
        }
    }
    set
}

/// Extract labels and their indices from `.aux` content.
pub fn collect_labels(aux: &str) -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    let re = Regex::new(r"\\newlabel\{(.+?)\}\{\{(.+?)\}").unwrap();
    for caps in re.captures_iter(aux) {
        map.insert(caps[1].to_string(), caps[2].to_string());
    }
    map
}
