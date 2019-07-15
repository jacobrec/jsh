pub fn parse(input: String) -> super::ast::Ast {
    println!("{}", input);
    return super::ast::new()
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
}
