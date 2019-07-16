pub fn lit(expected: &'static str) -> impl Fn(&str) -> Result<(&str, String), &str> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], (String::from(expected))))
        }
        _ => Err(input),
    }
}

fn and<P1, P2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, String), &str>
where
    P1: Fn(&str) -> Result<(&str, String), &str>,
    P2: Fn(&str) -> Result<(&str, String), &str>,
{
    move |input| {
        if let Ok(data) = parser1(input) {
            if let Ok(data2) = parser2(data.0) {
                Ok((data2.0, String::from(data.1 + &data2.1)))
            } else {
                Err(input)
            }
        } else {
            Err(input)
        }
    }
}

fn or<P1, P2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, String), &str>
where
    P1: Fn(&str) -> Result<(&str, String), &str>,
    P2: Fn(&str) -> Result<(&str, String), &str>,
{
    move |input| {
        if let Ok(data) = parser1(input) {
            Ok((data.0, data.1))
        } else {
            if let Ok(data) = parser2(input) {
                Ok((data.0, data.1))
            } else {
                Err(input)
            }
        }
    }
}

// and!(p1, p2, p3) -> and(p1, and!(p2, p3)) -> and(p1, and(p2, and!(p3))) -> and(p1, and(p2, p3))
macro_rules! and {
    ($w:expr) => ($w);
    ($w:expr, $($rest:tt)+) => (and($w, and!( $( $rest )+ )));
}

macro_rules! or {
    ($w:expr) => ($w);
    ($w:expr, $($rest:tt)+) => (or($w, or!( $( $rest )+ )));
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
        assert_eq!(data.1, "ba");

        let res = ba("apple");
        assert_eq!(res.expect_err("This test should fail"), "apple");
    }

    #[test]
    fn test_or_a() {
        let ba = or(lit("b"), lit("a"));

        let res = ba("banana");
        let data = res.expect("This test should pass");
        assert_eq!(data.0, "anana");
        assert_eq!(data.1, "b");

        let res = ba("apple");
        let data = res.expect("This test should pass");
        assert_eq!(data.0, "pple");
        assert_eq!(data.1, "a");

        let res = ba("orange");
        assert_eq!(res.expect_err("This test should fail"), "orange");
    }

    #[test]
    fn test_and_b() {
        let ban = and!(lit("b"), lit("a"), lit("n"));
        let res = ban("banana");
        let data = res.expect("This test should pass");
        assert_eq!(data.0, "ana");
        assert_eq!(data.1, ("b", "a", "n"));
    }
    #[test]
    fn test_or_b() {
        let abc = or!(lit("a"), lit("b"), lit("c"));

        let res = abc("apple");
        let data = res.expect("This test should pass");
        assert_eq!(data.0, "pple");
        assert_eq!(data.1, "a");

        let res = abc("banana");
        let data = res.expect("This test should pass");
        assert_eq!(data.0, "anana");
        assert_eq!(data.1, "b");

        let res = abc("orange");
        let data = res.expect_err("This test should fail");
        assert_eq!(data, "orange");
    }

}
