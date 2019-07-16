mod parser;
mod ast;

fn main() {
    let result = parser::parse(String::from("cal"));
    println!("{:#?}", result);
}
