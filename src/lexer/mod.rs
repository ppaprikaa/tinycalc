mod matchers;

use super::fsm;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum TokenKind {
    Undefined = 0,
    Number = 1,
    Operator = 2,
    Parenthesis = 3
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: String
}

impl Token {
    pub fn new() -> Token {
        Token {
            value: String::new(),
            kind: TokenKind::Undefined
        }
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }
}

pub struct Lexer { 
    matchers: Vec<(TokenKind, super::fsm::FSM)>
}

impl Lexer {
    pub fn new() -> Lexer {
        let mut matchers: Vec<(TokenKind, fsm::FSM)> = Vec::new();

        matchers.push((TokenKind::Number, matchers::new_number_matcher()));
        matchers.push((TokenKind::Operator, matchers::new_operator_matcher()));
        matchers.push((TokenKind::Parenthesis, matchers::new_parenthesis_matcher()));
        
        Lexer {
            matchers,
        }
    }

    pub fn analyze(&self, input: String) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars: Vec<char> = input.chars().rev().collect();
        let mut current_token = Token::new();
        let mut char_matches: bool;

        while let Some(char) = chars.pop() {
            if char == ' ' {
                continue;
            }

            char_matches = false;
            current_token.value.push(char);

            for (kind, matcher) in &self.matchers {
                let (matches, _) = matcher.matches(current_token.value.clone());
                if matches {
                    char_matches = true;     
                    current_token.kind = kind.clone();
                }

            }

            if !char_matches {
                if current_token.value.chars().count() <= 1 && char != ' ' {
                    return Err(format!("char {} doesn't match any token", char));
                }

                current_token.value.pop();
                if char != ' ' {
                    chars.push(char);
                }

                tokens.push(current_token);
                current_token = Token::new();
                continue;
            }
        }

        if current_token.kind > TokenKind::Undefined {
            tokens.push(current_token);
        }

        Ok(tokens)
    }
}
