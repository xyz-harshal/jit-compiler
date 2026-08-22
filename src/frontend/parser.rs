use crate::frontend::ast::Expr;

enum Ops {
    Add,
    Sub,
    Mul,
}

fn symbol_match(c: char) -> Result<Ops, String> {
    match c {
        '+' => Ok(Ops::Add),
        '-' => Ok(Ops::Sub),
        '*' => Ok(Ops::Mul),
        _ => Err("Not a operation".to_string()),
    }
}

pub fn parse(input: &str) -> Result<Expr, String> {
    let input: &str = input.trim();
    for i in (0..input.len()).rev() {
        let c = input.chars().nth(i).unwrap();
        if c == '-' && i == 0 { continue; }
        let res: Expr = match symbol_match(c) {
            Ok(ops) => {
                let (left, right) = input.split_at(i);
                let right = &right[1..];
                match ops {
                    Ops::Add => Expr::Add(Box::new(parse(left)?), Box::new(parse(right)?)),
                    Ops::Sub => Expr::Sub(Box::new(parse(left)?), Box::new(parse(right)?)),
                    Ops::Mul => Expr::Mul(Box::new(parse(left)?), Box::new(parse(right)?)),
                }
            },
            _ => continue,
        };
        return Ok(res);
    }
    Ok(Expr::Number(input.parse::<i64>().map_err(|_| "Not a Number".to_string())?))
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::frontend::ast::Expr;

    #[test]
    fn parses_number() {
        assert_eq!(parse("42"), Ok(Expr::Number(42)));
    }

    #[test]
    fn parses_number_with_whitespace() {
        assert_eq!(parse("  42  "), Ok(Expr::Number(42)));
    }

    #[test]
    fn rejects_non_number() {
        assert_eq!(parse("abc"), Err("Not a Number".to_string()));
    }

    #[test]
    fn parses_addition() {
        assert_eq!(
            parse("4 + 5"),
            Ok(Expr::Add(
                Box::new(Expr::Number(4)),
                Box::new(Expr::Number(5))
            ))
        );
    }

    #[test]
    fn parses_mixed_operators_left_associative() {
        assert_eq!(
            parse("10 - 3 + 2"),
            Ok(Expr::Add(
                Box::new(Expr::Sub(
                    Box::new(Expr::Number(10)),
                    Box::new(Expr::Number(3))
                )),
                Box::new(Expr::Number(2))
            ))
        );
    }
    #[test]
    fn parses_negative_number() {
        assert_eq!(parse("-5"), Ok(Expr::Number(-5)));
    }
}
