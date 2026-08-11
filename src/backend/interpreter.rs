use crate::frontend::ast::Expr;

//Executing the AST tree into numerical
pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(left, right) => {
            eval(left) + eval(right)
        },
        Expr::Sub(left, right) => {
            eval(left) - eval(right)
        },
    }
}
