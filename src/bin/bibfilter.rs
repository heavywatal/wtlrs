use clap::Parser;
use std::fs;
use std::path::PathBuf;
use wtlrs::{bibtex, latex};

/// Filter bibtex file with citekeys in aux file.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// If not specified, defaults to stdout
    #[clap(short, long)]
    outfile: Option<PathBuf>,
    /// Path to bibtex file
    bib: PathBuf,
    /// Path to aux file
    aux: PathBuf,
}

fn main() {
    let args = Args::parse();
    let aux = fs::read_to_string(args.aux).unwrap();
    let citekeys = latex::collect_citekeys(&aux);
    // eprintln!("{:?}", citekeys);
    let mut content = fs::read_to_string(args.bib).unwrap();
    content = bibtex::filter(&content, &citekeys);
    if let Some(outfile) = args.outfile {
        fs::write(outfile, content).expect("error");
    } else {
        print!("{}", content);
    }
}
