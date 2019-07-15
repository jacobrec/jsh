pub fn parse(input: String) -> Box<super::ast::Ast> {
    return Box::from(super::ast::Token{ value: input })
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
