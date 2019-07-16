pub fn parse_atom(input: String) -> Box<crate::ast::Ast> {
    if let Some(val) = parse_string(input) {
        Box::from(crate::ast::Token{ value: val })
    } else {
        Box::from(crate::ast::Token{ value: String::from("error") })
    }
}

pub fn parse_string(input: String) -> Option<String> {
    if input.starts_with("\"") {
        input.get(1..).expect("The first char could not be removed").find('\"')
            .and_then(|x| input.get(1..(x + 1)))
            .and_then(|val| Some(String::from(val)))
    } else {
        None
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string(String::from("'Hello, world'")), None);
        assert_eq!(parse_string(String::from("\"Hello, world\"")), Some(String::from("Hello, world")));
        assert_eq!(parse_string(String::from("\"Hello, world\" hahah ''")), Some(String::from("Hello, world")));
    }
}
