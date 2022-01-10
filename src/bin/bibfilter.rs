use std::env;
use std::fs;
use std::path::Path;
use wtlrs::{bibtex, latex};

fn main() {
    let args: Vec<String> = env::args().collect();
    let bibfile = Path::new(&args[1]);
    let auxfile = Path::new(&args[2]);
    let aux = fs::read_to_string(auxfile).unwrap();
    let citekeys = latex::collect_citekeys(&aux);
    // eprintln!("{:?}", citekeys);
    let mut content = fs::read_to_string(bibfile).unwrap();
    content = bibtex::filter(&content, &citekeys);
    print!("{}", content);
}
