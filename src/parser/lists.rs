use super::atoms::*;
use super::utils::*;
use crate::ast::*;

// Parses one list starting with (, or one atom
pub fn parse_s_expr(iinput: String) -> (SExp, String) {
    let input = String::from(iinput.trim());
    if input.starts_with("()") {
        (SExp::SAtom(Atom::ANil), clip_and_clean(input, 2))
    } else if input.starts_with("(") {
        parse_s_expr_inner(clip_and_clean(input, 1))
    } else {
        match parse_atom(clean(input)) {
            Ok((a, s)) => (SExp::SAtom(a), s),
            Err(s) => (SExp::SAtom(Atom::ANil), s),
        }
    }
}

fn parse_s_expr_inner(input: String) -> (SExp, String) {
    if input.starts_with(")") {
        (SExp::SAtom(Atom::ANil), clip_and_clean(input, 1))
    } else {
        let (next, next_str) = parse_s_expr(input);
        let (rest, rest_string) = parse_s_expr_inner(next_str);
        (SExp::SPair(Box::new(next), Box::new(rest)), rest_string)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_parse_s_expr() {
        let result = parse_s_expr(String::from("( cal )"));
        assert_eq!(result.0, SExp::SPair(Box::from(SExp::SAtom(Atom::AString(String::from("cal")))), Box::from(SExp::SAtom(Atom::ANil))));
        assert_eq!(result.1, String::from(""));
    }
}
