use std::path::Path;
use std::env;

pub fn get_builtin(cmd: &String) -> Option<fn(crate::ast::SExp) -> String> {
    match cmd.as_ref() {
        "cd" => Some(builtin_cd),
        "pipe" => Some(builtin_pipe),
        _ => None
    }
}

fn builtin_cd(args: crate::ast::SExp) -> String {
    let new_dir = match args {
        crate::ast::SExp::SAtom(crate::ast::Atom::ANil) => String::from("/"),
        crate::ast::SExp::SAtom(crate::ast::Atom::AString(_)) => panic!("Malformed S-Expression"),
        crate::ast::SExp::SPair(a, b) => {
            if *b != crate::ast::SExp::SAtom(crate::ast::Atom::ANil) {
                String::from("cd: too many arguments")
            } else {
                super::eval_inner(*a)
            }
        },
    };
    let root = Path::new(new_dir.as_str());
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
    String::from("")
}

fn builtin_pipe(args: crate::ast::SExp) -> String {
    let mut prgms = Vec::new();
    let mut node = args;
    while let crate::ast::SExp::SPair(prgm, tail) = node {
        node = *tail;
        if let crate::ast::SExp::SPair(head, tail) = *prgm {
            if let crate::ast::SExp::SAtom(crate::ast::Atom::AString(cmd)) = *head {
                prgms.push((cmd, *tail));
            }
        }
    }
    super::eval_pipe_chain(prgms)
}
