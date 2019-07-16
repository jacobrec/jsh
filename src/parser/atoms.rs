pub fn parse_atom(input: String) -> Option<crate::ast::Atom> {
    parse_token(input).and_then(|tok| Some(crate::ast::Atom::AString(tok)))
}

pub fn parse_token(input: String) -> Option<String> {
    input.find(' ')
        .and_then(|x| input.get(0..x))
        .and_then(|val| Some(String::from(val)))
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
    fn test_parse_token() {
        let tbox = parse_atom(String::from("Hello world"));
        assert_eq!(tbox.expect("this should pass"), crate::ast::Atom::AString(String::from("Hello")));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string(String::from("'Hello, world'")), None);
        assert_eq!(parse_string(String::from("\"Hello, world\"")), Some(String::from("Hello, world")));
        assert_eq!(parse_string(String::from("\"Hello, world\" hahah ''")), Some(String::from("Hello, world")));
    }
}
