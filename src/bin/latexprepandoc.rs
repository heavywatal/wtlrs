use std::env;
use std::fs;
use std::path::Path;
use wtlrs::latex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let texfile = Path::new(&args[1]);
    let auxfile = texfile.with_extension("aux");
    let aux = fs::read_to_string(&auxfile).expect("error");
    let labelmap = latex::collect_labels(&aux);
    eprintln!("{:?}", labelmap);
    let mut content = fs::read_to_string(texfile).expect("error");
    content = latex::resolve_ref(&content, &labelmap);
    content = latex::remove_asterisk(&content);
    content = latex::label_caption(&content, &labelmap);
    print!("{}", content);
}
