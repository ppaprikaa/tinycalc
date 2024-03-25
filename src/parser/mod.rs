use crate::lexer;

#[derive(Debug, Clone)]
pub enum Operator {
    Undefined,
    Mul,
    Div,
    Plus,
    Minus
}

impl Operator {
    fn to_string(&self) -> String {
            match self {
                Operator::Mul => String::from("Mul"),
                Operator::Div => String::from("Div"),
                Operator::Plus => String::from("Plus"),
                Operator::Minus => String::from("Minus"),
                Operator::Undefined => String::from("?")
            }
    }
}

#[derive(Debug)]
pub enum Parentheses {
    Opening,
    Closing,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Undefined,
    Finished,
    Number(String),
    Binary {
        op: Operator,
        lhs: Box<Expression>,
        rhs: Box<Expression>
    },
    Unary {
        op: Operator,
        child: Box<Expression>,
    },
    Group {
        child: Box<Expression>
    }
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Number(num) => format!("{}", num),
            Expression::Binary { op, lhs, rhs } => format!("{}( {}, {} )", op.to_string(), lhs.to_string(), rhs.to_string()),
            Expression::Unary { op, child } => format!("{}( {} )", op.to_string(), child.to_string()),
            Expression::Group { child } => format!("GROUP( {} )", child.to_string()),
            Expression::Undefined => String::from("???"),
            Expression::Finished => String::from("FF"),
        }
    }
}

pub struct Parser {
    tokens: Vec<lexer::Token>,
    group: bool,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Parser {
        Parser {
            tokens,
            group: false,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        let mut current = Expression::Undefined;
        let mut new_node: Result<Expression, String>;

        while self.pos < self.tokens.len() {
            if self.group {
                match current {
                    Expression::Group { .. } => {
                        self.group = false;
                        continue;
                    },
                    Expression::Undefined => (),
                    _ => {
                        return Ok(current)
                    },
                }
            }

            new_node = self.parse_one();
            match new_node {
                Ok(node) => {
                    match node {
                        Expression::Number(num) => {
                            current = Expression::Number(num);
                        },
                        Expression::Unary { .. } => { 
                            current = node;
                        },
                        Expression::Binary { op, rhs, .. } => {
                            current = Expression::Binary { op, lhs: Box::new(current), rhs }
                        },
                        Expression::Group { .. } => { 
                            current = node;
                        }
                        Expression::Finished => break,
                        _ => return Err(format!("failed to parse node {}", node.to_string()))
                    }
                },
                Err(err) => return Err(err),
            }
        }

        if let Expression::Binary { op, lhs, rhs } = current.clone() {
            if let Expression::Binary { op: rop, lhs: rlhs, rhs: rrhs } = *rhs {
                match op {
                    Operator::Mul | Operator::Div => {
                        match rop {
                            Operator::Minus | Operator::Plus => {
                                let new_lhs = Expression::Binary { op, lhs, rhs: rlhs };
                                return Ok(Expression::Binary { op: rop, lhs: Box::new(new_lhs), rhs: rrhs });
                            }
                            _ => (),
                        } 
                    }
                    _ => (),
                }
            }
        }

        Ok(current)
    }

    pub fn parse_one(&mut self) -> Result<Expression, String> {
        let kind = self.tokens[self.pos].kind();
        
        match kind {
            lexer::TokenKind::Number => {
                self.parse_number()
            },
            lexer::TokenKind::Operator => {
                let prev_token_pos = if self.pos > 0 {
                    self.pos - 1
                } else {
                    0
                };

                if let Some(prev_token) = self.tokens.get(prev_token_pos) {
                    match prev_token.kind() {
                        lexer::TokenKind::Parenthesis => {
                            if prev_token.value() == ")" {
                                return self.parse_binary_expr()
                            }
                        },
                        lexer::TokenKind::Number => return self.parse_binary_expr(),
                        _ => ()
                    }
                }

                self.parse_unary_expr()
            },
            lexer::TokenKind::Parenthesis => {
                self.parse_parenthesis()
            },
            _ => Err(format!("there's no parsing instruction for token kind {:?}", kind))
        }
    }

    pub fn parse_number(&mut self) -> Result<Expression, String> {
        let new_expr: Expression;

        if let lexer::TokenKind::Number = self.tokens[self.pos].kind() {
            new_expr = Expression::Number(self.tokens[self.pos].value());
        } else {
            new_expr = Expression::Undefined;
        }
        self.pos += 1;

        Ok(new_expr)
    }

    pub fn parse_binary_expr(&mut self) -> Result<Expression, String> {
        let operator: Operator;
        match &self.tokens[self.pos].value()[..] {
            "-" => operator = Operator::Minus,
            "+" => operator = Operator::Plus,
            "*" => operator = Operator::Mul,
            "/" => operator = Operator::Div,
            _ => {
                return Err(format!("operator {} not defined", self.tokens[self.pos].value()));
            },
        };

        self.pos += 1;
        match self.parse() {
            Ok(rhs) => Ok(Expression::Binary { 
                op: operator, 
                lhs: Box::new(Expression::Undefined), 
                rhs: Box::new(rhs) 
            }),
            Err(err) => Err(format!("{}", err))

        }
    }

    pub fn parse_unary_expr(&mut self) -> Result<Expression, String> {
        let operator: Operator;
        match &self.tokens[self.pos].value()[..] {
            "-" => operator = Operator::Minus,
            "+" => operator = Operator::Plus,
            _ => {
                return Err(format!("{} cannot be unary operator", self.tokens[self.pos].value()));
            },
        };
        self.pos += 1;
        
        let child = self.parse_one();
        match child {
            Ok(child) => {
                match child {
                    Expression::Number(_) => (),
                    Expression::Group {..} => {
                        self.group = false;
                    },
                    _ => return Err(format!("child {} cannot be child of unary operator", child.to_string())),
                }
                Ok(Expression::Unary { op: operator, child: Box::new(child) })
            }
            Err(err) => Err(format!("parsing child failed with error: {}", err)),
        }
    }

    pub fn parse_parenthesis(&mut self) -> Result<Expression, String> {
        let mut new_expr = Expression::Undefined;

        if let lexer::TokenKind::Parenthesis = self.tokens[self.pos].kind()  {
            let parenthesis = &self.tokens[self.pos].value()[..];
            match parenthesis {
                "(" => { 
                    self.pos += 1;
                    match self.parse() {
                        Ok(node) => {
                            new_expr = Expression::Group {child: Box::new(node)};
                        },
                        Err(err) => return Err(err),
                    }

                    if let Expression::Group { ref child, ..} = new_expr {
                        if let Expression::Undefined = **child {
                            return Err(String::from("undefined child of group node")); 
                        } 
                    }

                    return Ok(new_expr);
                },
                ")" => { 
                    self.pos += 1;
                    self.group = true;
                    return Ok(Expression::Finished);
                },
                _ => return Err(format!("parenthesis {} is not defined", parenthesis)),
            }
        }
        Ok(new_expr)
    }
}
