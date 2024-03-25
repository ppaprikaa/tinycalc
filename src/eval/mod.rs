use std::fmt;

use crate::parser;

#[derive(Debug, Clone)]
pub struct EvalError {
    pub msg: String,
}

impl EvalError {
    fn new(msg: String) -> EvalError {
        EvalError { msg }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "eval error: {}", self.msg)
    }
}

pub fn eval(expr: parser::Expression) -> Result<f64, EvalError> {
    match expr.clone() {
        parser::Expression::Number(num) => {
            match num.parse::<f64>() {
                Ok(num) => Ok(num),
                Err(err) => {
                    Err(EvalError::new(format!("failed to parse number: {}\n err: {}", num, err)))
                },
            }
        },
        parser::Expression::Group { child } => {
            eval(*child)
        },
        parser::Expression::Unary { op, child } => {
            let sign = if let parser::Operator::Minus = op { -1.0 } else { 1.0 };
            match eval(*child) {
                Ok(num) => Ok(num * sign),
                Err(err) => Err(EvalError::new(format!("failed to parse unary expr: {} with error: {}", expr.to_string(), err)))
            }
        },
        parser::Expression::Binary { op, lhs, rhs } => {
            let lhs_val = eval(*lhs.clone());
            let rhs_val = eval(*rhs.clone());

            match lhs_val {
                Err(err) => Err(EvalError::new(format!("failed to parse subexpr {} of expr {} with error: {}", lhs.to_string(), expr.to_string(), err))),
                Ok(lhs) => {
                    match rhs_val {
                        Err(err) => Err(EvalError::new(format!("failed to parse subexpr {} of expr {} with error: {}", rhs.to_string(), expr.to_string(), err))),
                        Ok(rhs) => {
                            match op {
                                parser::Operator::Mul => Ok(lhs * rhs),
                                parser::Operator::Div => {
                                    if lhs == 0.0 {
                                        return Err(EvalError::new(format!("right value of expr {} is zero", expr.to_string())));
                                    }
                                    Ok(lhs / rhs)
                                },
                                parser::Operator::Plus => Ok(lhs + rhs),
                                parser::Operator::Minus => Ok(lhs - rhs),
                                _ => Err(EvalError::new(format!("invalid operator at expr {}", expr.to_string()))),
                            }
                        }
                    }
                }
            }
        }
        _ => Err(EvalError::new(format!("failed to parse expression: {} unknown error", expr.to_string()))),
    }
}
