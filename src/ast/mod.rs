#[derive(Debug)]
pub enum SExp {
    SAtom(Atom),
    SPair(Box<SExp>, Box<SExp>)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Atom {
    AString(String),
    AInt(i32),
    AFloat(f32),
    ABool(bool),
    ANil()
}
