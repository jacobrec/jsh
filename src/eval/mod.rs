use crate::ast;
use crate::ast::SExp::{SPair, SAtom};
use crate::ast::Atom::{ANil, AString};
use std::process::Command;

pub fn display(result: &ast::SExp) -> String{
    match result {
        SPair(a, b) => format!("({} . {})", display(&*a), display(&*b)),
        SAtom(AString(s)) => format!("{}", s),
        SAtom(ANil) => String::from("()"),
    }
}

pub fn eval(result: ast::SExp) {
    println!("{}", eval_inner(result));
}

fn eval_inner(result: ast::SExp) -> String {
    match result {
        SPair(a, b) => eval_command(*a, *b),
        SAtom(AString(s)) => s,
        SAtom(ANil) => String::from(""),
    }
}

fn flatten(tail: ast::SExp) -> Vec<String> {
    let mut node = tail;
    let mut res = Vec::new();
    while node != SAtom(ANil) {
        match node {
            SPair(a, b) => {
                node = *b;
                match *a {
                    SPair(a, b) => res.push(eval_inner(SPair(a, b))),
                    SAtom(AString(s)) => res.push(s),
                    SAtom(ANil) => panic!("s expression contains nil at head"),
                }
            },
            SAtom(AString(s)) => {
                res.push(s);
                node = SAtom(ANil);
            },
            SAtom(ANil) => panic!("This should never occur")
        };
    }

    res
}

fn eval_command(head: ast::SExp, tail: ast::SExp) -> String {
    if let SAtom(AString(cmd)) = head {
        let sub_process = Command::new(cmd)
                .args(flatten(tail))
                .output()
                .expect("Failed to run command");
        String::from(std::str::from_utf8(&sub_process.stdout).expect("Failed to process program output"))
    } else {
        panic!("Head is not a string")
    }
}
