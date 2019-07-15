mod parser;
mod ast;

fn main() {
    parser::parse(String::from("Hello, World!"));
}
