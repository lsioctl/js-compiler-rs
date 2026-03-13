#[derive(Debug)]
enum MathExpr {
    Plus {
        left: Box<MathExpr>,
        right: Box<MathExpr>
    },
    Minus {
        left: Box<MathExpr>,
        right: Box<MathExpr>
    },
    Value(i32)
}

impl MathExpr {
    fn eval(&self) -> i32 {
        match self {
            MathExpr::Plus{ left, right } => left.eval() + right.eval(),
            MathExpr::Minus{ left, right } => left.eval() - right.eval(),
            MathExpr::Value(a) => *a,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {

        let m1 = MathExpr::Plus {
            left: Box::new(MathExpr::Value(3)),
            right: Box::new(MathExpr::Value(2))
        };

        assert_eq!(m1.eval(), 5);

        let m2 = MathExpr::Minus {
            left: Box::new(MathExpr::Value(25)),
            right: Box::new(m1)
        };

        assert_eq!(m2.eval(), 20);


    }
}