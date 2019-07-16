// Parses one list starting with (, or one atom
pub fn parse_s_expr(input: String) -> Box<crate::ast::Ast> {
    parse_s_expr_inner(input)
}

// parses 0 or more s-exp stopping at EOF or )
pub fn parse_s_expr_inner(input: String) -> Box<crate::ast::Ast> {
    Box::from(crate::ast::Token{ value: input })
}
