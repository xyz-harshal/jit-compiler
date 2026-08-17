use crate::frontend::ast::Expr;

pub fn parse(input: &str) -> Result<Expr, String> {

    let input: &str = input.trim();
    let add: Option<usize> = input.rfind('+');
    let sub: Option<usize> = input[1..].rfind('-').map(|idx| idx + 1);

    //when you write Some(a) in a match pattern then it checks if Option is an Some(a) if yes then the value gets extracted inside and call it a.
    let res: Expr = match (add, sub) {
        (Some(a), Some(s)) => {
            let x: usize = a.max(s);
            let (left, right) : (&str, &str) = input.split_at(x);
            let right: &str = &right[1..];
            match a > s {
                true => Expr::Add(Box::new(parse(left)?), Box::new(parse(right)?)),
                false => Expr::Sub(Box::new(parse(left)?), Box::new(parse(right)?)),
            }
        },
        (Some(a), None) => {
            let (left, right) : (&str, &str) = input.split_at(a);
            let right: &str = &right[1..];
            Expr::Add(Box::new(parse(left)?), Box::new(parse(right)?))
        },
        (None, Some(s)) => {
            let (left, right) : (&str, &str) = input.split_at(s);
            let right: &str = &right[1..];
            Expr::Sub(Box::new(parse(left)?), Box::new(parse(right)?))
        },
        (None, None) => Expr::Number(input.parse::<i64>().map_err(|_| "Not a Number".to_string())?),
    };
    Ok(res)
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
