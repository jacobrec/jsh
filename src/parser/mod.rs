mod combinators;

pub fn parse(input: String) -> Box<super::ast::Ast> {
    parse_s_expr(input)
}

fn parse_s_expr(input: String) -> Box<super::ast::Ast> {
    Box::from(super::ast::Token{ value: input })
}

fn parse_token(input: String) -> Box<super::ast::Ast> {
    Box::from(super::ast::Token{ value: parse_string(input) })
}

fn parse_string(input: String) -> String {
    input
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    #[ignore]
    fn e2e_test_1() {
        parse(String::from("echo hello"));
    }

    #[test]
    #[ignore]
    fn e2e_test_2() {
        parse(String::from("echo (+ he llo)"));
    }

    #[test]
    #[ignore]
    fn e2e_test_3() {
        parse(String::from("echo '(+ he llo)'"));
    }

    #[test]
    fn e2e_test_4() {
        let result = parse(String::from("cal"));
        assert_eq!(result.eval(), "cal");
    }
}
