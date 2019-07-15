use std::fmt;

pub trait Ast: fmt::Display {
    fn eval(&self) -> String;
}

pub struct Cmd {
    pub function: String,
    pub args: Vec<Box<Cmd>>
}
impl Ast for Cmd {
    fn eval(&self) -> String {
        self.function.clone()
    }
}
impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} ...)", self.function)
    }
}

pub struct Token {
    pub value: String,
}
impl Ast for Token {
    fn eval(&self) -> String {
        self.value.clone()
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.value)
    }
}
