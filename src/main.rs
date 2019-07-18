use std::io::{self, Write, stdout};

mod parser;
mod ast;
mod eval;

fn main() {
    loop {
        prompt();
        let val = read();
        match val {
            Ok(input) => {
                eval::eval(parser::parse(input));
            },
            Err(0) => { println!(); break },
            Err(_) => ()
        }
    }
}

fn prompt () {
    print!("jsh> ");
    let mut is_flushed = false;
    while !is_flushed  {
        let res = stdout().flush();
        if let Ok(_) = res {
            is_flushed = true;
        }
    }
}

fn read () -> Result<String, u8> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n > 0 {
                Ok(input)
            } else {
                Err(0)
            }
        }
        Err(_) => Err(1),
    }
}

