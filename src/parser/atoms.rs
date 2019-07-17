use super::utils::*;

pub fn parse_atom(input: String) -> Result<(crate::ast::Atom, String), String> {
    if input.starts_with("\"") {
        parse_string(input).and_then(|tok| Ok((crate::ast::Atom::AString(tok.0), tok.1)))
    } else {
        parse_token(input).and_then(|tok| Ok((crate::ast::Atom::AString(tok.0), tok.1)))
    }
}

pub fn parse_token(iinput: String) -> Result<(String, String), String> {
    let mut input = iinput.clone();
    let val = find_first(&input, vec![String::from(" "), String::from("("), String::from(")")]);
    val.and_then(|x| { let rest = input.split_off(x); Some((input, rest)) })
        .and_then(|(val, rest)| Some((String::from(val), clean(rest))))
        .ok_or(iinput)
}

pub fn parse_string(iinput: String) -> Result<(String, String), String> {
    let mut input = iinput.clone();
    if input.starts_with("\"") {
        input.get(1..).expect("The first char could not be removed").find('\"')
            .and_then(|x| { let rest = input.split_off(x + 2); Some((input.get(1..(input.len()-1)).expect("Failed to remove quotes"), rest)) })
            .and_then(|(val, rest)| Some((String::from(val), clean(rest))))
            .ok_or(iinput)
    } else {
        Err(iinput)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_parse_atom() {
        let tbox = parse_atom(String::from("Hello world"));
        let t = tbox.expect("This should pass");
        assert_eq!(t.0, crate::ast::Atom::AString(String::from("Hello")));
        assert_eq!(t.1, String::from("world"));

        let tbox = parse_atom(String::from("\"Hello world\" again"));
        let t = tbox.expect("This should pass");
        assert_eq!(t.0, crate::ast::Atom::AString(String::from("Hello world")));
        assert_eq!(t.1, String::from("again"));
    }

    #[test]
    fn test_parse_token() {
        let tbox2 = parse_token(String::from("Hello)"));
        let t2 = tbox2.expect("This should pass");

        assert_eq!(t2.0, String::from("Hello"));
        assert_eq!(t2.1, String::from(")"));

        let tbox = parse_token(String::from("Hello world"));
        let t = tbox.expect("This should pass");

        assert_eq!(t.0, String::from("Hello"));
        assert_eq!(t.1, String::from("world"));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string(String::from("'Hello, world'")), Err(String::from("'Hello, world'")));
        assert_eq!(parse_string(String::from("\"Hello, world\"")), Ok((String::from("Hello, world"), String::from(""))));
        assert_eq!(parse_string(String::from("\"Hello, world\" hahah ''")), Ok((String::from("Hello, world"), String::from("hahah ''"))));
    }
}
