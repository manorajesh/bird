#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    fn from(word: &str) -> Token {
        match word {
            "+" | "-" | "*" | "/" | "=" => Token {
                kind: TokenKind::Operator,
                value: word.to_string(),
            },

            _ => {
                if word.is_literal() {
                    Token {
                        kind: TokenKind::Literal,
                        value: word.to_string(),
                    }
                } else {
                    Token {
                        kind: TokenKind::Identifier,
                        value: word.to_string(),
                    }
                }
            }
        }
    }
}

trait IsLiteral {
    fn is_literal(&self) -> bool;
}

impl IsLiteral for str {
    fn is_literal(&self) -> bool {
        // number type
        if self.parse::<f64>().is_ok() {
            return true;
        } 
        
        // string and char type
        if self.contains("\"") | self.contains("'") {
            return true;
        }

        false
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Operator,
    Literal,
    Identifier,
}

pub fn lex(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let stripped = input.split_whitespace();
    for word in stripped {
        let word = word.trim_end_matches(';');
        tokens.push(Token::from(word));
    }

    tokens
}