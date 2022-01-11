use clap::Parser;
use std::fs;
use std::path::PathBuf;
use wtlrs::latex;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    outfile: Option<PathBuf>,
    tex: PathBuf,
}

fn main() {
    let args = Args::parse();
    let auxfile = args.tex.with_extension("aux");
    let aux = fs::read_to_string(&auxfile).expect("error");
    let labelmap = latex::collect_labels(&aux);
    eprintln!("{:?}", labelmap);
    let mut content = fs::read_to_string(args.tex).expect("error");
    content = latex::resolve_ref(&content, &labelmap);
    content = latex::remove_asterisk(&content);
    content = latex::label_caption(&content, &labelmap);
    if let Some(outfile) = args.outfile {
        fs::write(outfile, content).expect("error");
    } else {
        print!("{}", content);
    }
}
