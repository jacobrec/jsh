mod lists;
mod atoms;
mod utils;

pub fn parse(input: String) -> crate::ast::SExp {
    lists::parse_s_expr(clean_input(input)).0
}

fn clean_input(input: String) -> String {
    format!("({})", input)
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn e2e_test_1() {
        let result = parse(String::from("echo hello"));
        assert_eq!(result, crate::ast::SExp::SPair(
                Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::AString(String::from("echo")))),
                Box::from(crate::ast::SExp::SPair(
                        Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::AString(String::from("hello")))),
                        Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::ANil))))));
    }

    #[test]
    fn e2e_test_2() {
        let result = parse(String::from("echo (+ he llo)"));
        assert_eq!(result, crate::ast::SExp::SPair(
                Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::AString(String::from("echo")))),
                Box::from(crate::ast::SExp::SPair(
                    Box::from(crate::ast::SExp::SPair(
                        Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::AString(String::from("+")))),
                        Box::from(crate::ast::SExp::SPair(
                            Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::AString(String::from("he")))),
                            Box::from(crate::ast::SExp::SPair(
                                Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::AString(String::from("llo")))),
                                Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::ANil)))))))),
                    Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::ANil))))));
    }

    #[test]
    fn e2e_test_3() {
        let result = parse(String::from("echo \"(+ he llo)\""));
        assert_eq!(result, crate::ast::SExp::SPair(
                Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::AString(String::from("echo")))),
                Box::from(crate::ast::SExp::SPair(
                        Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::AString(String::from("(+ he llo)")))),
                        Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::ANil))))));
    }

    #[test]
    fn e2e_test_4() {
        let result = parse(String::from("cal"));
        assert_eq!(result, crate::ast::SExp::SPair(
                Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::AString(String::from("cal")))),
                Box::from(crate::ast::SExp::SAtom(crate::ast::Atom::ANil))));
    }
}
