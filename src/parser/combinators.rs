pub fn lit(expected: &'static str) -> impl Fn(&str) -> Result<(&str, String), &str> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], (String::from(expected))))
        }
        _ => Err(input),
    }
}

fn and<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
where
    P1: Fn(&str) -> Result<(&str, R1), &str>,
    P2: Fn(&str) -> Result<(&str, R2), &str>,
{
    move |input| {
        if let Ok(data) = parser1(input) {
            if let Ok(data2) = parser2(data.0) {
                Ok((data2.0, (data.1, data2.1)))
            } else {
                Err(input)
            }
        } else {
            Err(input)
        }
    }
}

fn or<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, (Option<R1>, Option<R2>)), &str>
where
    P1: Fn(&str) -> Result<(&str, R1), &str>,
    P2: Fn(&str) -> Result<(&str, R2), &str>,
{
    move |input| {
        if let Ok(data) = parser1(input) {
            Ok((data.0, (Some(data.1), None)))
        } else {
            if let Ok(data) = parser2(input) {
                Ok((data.0, (None, Some(data.1))))
            } else {
                Err(input)
            }
        }
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_char_a() {
        let a = lit("a");
        let res = a("apple");
        assert_eq!(res.expect("This test should pass").0, "pple");

        let res = a("banana");
        assert_eq!(res.expect_err("This test should fail"), "banana");
    }

    #[test]
    fn test_char_b() {
        let b = lit("b");
        let res = b("banana");
        assert_eq!(res.expect("This test should pass").0, "anana");

        let res = b("apple");
        assert_eq!(res.expect_err("This test should fail"), "apple");
    }

    #[test]
    fn test_and_a() {
        let ba = and(lit("b"), lit("a"));

        let res = ba("banana");
        let data = res.expect("This test should pass");
        assert_eq!(data.0, "nana");
        assert_eq!(data.1, ("b".to_string(), "a".to_string()));

        let res = ba("apple");
        assert_eq!(res.expect_err("This test should fail"), "apple");
    }

    #[test]
    fn test_or_a() {
        let ba = or(lit("b"), lit("a"));

        let res = ba("banana");
        let data = res.expect("This test should pass");
        assert_eq!(data.0, "anana");
        assert_eq!(data.1, (Some("b".to_string()), None));

        let res = ba("apple");
        let data = res.expect("This test should pass");
        assert_eq!(data.0, "pple");
        assert_eq!(data.1, (None, Some("a".to_string())));

        let res = ba("orange");
        assert_eq!(res.expect_err("This test should fail"), "orange");
    }

}
