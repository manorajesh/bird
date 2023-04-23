use std::{env, fs::OpenOptions, io::Read};

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut file = OpenOptions::new()
        .write(false)
        .create(false)
        .read(true)
        .open(path)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let analysis = lexer::lex(contents);
    let ast = parser::parse(analysis);

    println!("{:#?}", ast);
}