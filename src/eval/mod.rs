use crate::ast;
use crate::ast::SExp::{SPair, SAtom};
use crate::ast::Atom::{ANil, AString};
use std::process::{Command, Stdio, Child};
use std::io::Read;

mod builtins;

pub fn display(result: &ast::SExp) -> String{
    match result {
        SPair(a, b) => format!("({} . {})", display(&*a), display(&*b)),
        SAtom(AString(s)) => format!("{}", s),
        SAtom(ANil) => String::from("()"),
    }
}

pub fn eval(result: ast::SExp) {
    if false {
        display(&result);
    }
    print!("{}", eval_inner(result));
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
        if let Some(builtin) = builtins::get_builtin(&cmd) {
            builtin(tail)
        } else {
            eval_prgm(cmd, tail, Stdio::piped(), Stdio::piped(), Stdio::piped())
        }
    } else {
        panic!("Head is not a string")
    }
}

fn eval_prgm(cmd: String, tail: ast::SExp, std_in: std::process::Stdio, std_out: std::process::Stdio, std_err: std::process::Stdio) -> String {
    let cmd_copy = cmd.clone();
    let sub_process = Command::new(cmd)
        .args(flatten(tail))
        .stdin(std_in)
        .stdout(std_out)
        .stderr(std_err)
        .output();

    if let Ok(output) = sub_process {
        String::from(std::str::from_utf8(&output.stdout).expect("Failed to process program output"))
    } else {
        format!("Command not found: {}", cmd_copy)
    }
}

fn eval_pipe_chain(programs: Vec<(String, ast::SExp)>) -> String {
    let mut prgms = programs.into_iter();
    let mut previous = None;
    while let Some((cmd, tail)) = prgms.next() {
        let cmd_copy = cmd.clone();
        let process = Command::new(cmd)
            .args(flatten(tail))
            .stdin(previous
                   .and_then(|c: Child| Some(c.stdout))
                   .and_then(|std| Some(Stdio::from(std.unwrap())))
                   .unwrap_or(Stdio::piped()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        if let Ok(output) = process {
            previous = Some(output)
        } else {
            return format!("Command not found: {}", cmd_copy)
        }
    }
    if let Some(mut final_command) = previous {
        final_command.wait();
        let mut buf = String::new();
        final_command.stdout.and_then(|mut std| Some(std.read_to_string(&mut buf)));
        buf
    } else {
        String::from("Error?")
    }
}

