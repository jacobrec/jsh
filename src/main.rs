extern crate rustyline;

mod input;
mod parser;
mod ast;
mod eval;

fn main() {
    let history_file = ".jsh_history";
    let mut rl = rustyline::Editor::<()>::new();
    if rl.load_history(history_file).is_err() {
        println!("No previous history.");
    }

    loop {
        let val = input::read(&mut rl);
        match val {
            Ok(input) => {
                eval::eval(parser::parse(input));
            },
            Err(1) => { break },
            Err(_) => ()
        }
    }
    rl.save_history(history_file).unwrap();

}



