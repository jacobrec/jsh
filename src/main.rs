mod parser;
mod ast;
mod eval;

fn main() {
    let result = parser::parse(String::from("echo hello world (echo hello again)"));
    //println!("{}", eval::display(&result));
    eval::eval(result);
}
