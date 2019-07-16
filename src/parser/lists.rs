// Parses one list starting with (, or one atom
pub fn parse_s_expr(input: String) -> crate::ast::SExp {
    parse_s_expr_inner(input)
}

// parses 0 or more s-exp stopping at EOF or )
pub fn parse_s_expr_inner(input: String) -> crate::ast::SExp {
    crate::ast::SExp::SAtom(crate::ast::Atom::AString(input))
}
