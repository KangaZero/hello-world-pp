use syntect::parsing::SyntaxSet;

fn main() {
    let ss = SyntaxSet::load_defaults_newlines();
    for s in ss.syntaxes() {
        // println!("cargo:warning={} {:?}", s.name, s.file_extensions);
        println!("cargo:warning={:?}", s);
    }
}
