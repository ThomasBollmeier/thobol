use alteraparser::prelude::Token;
use alteraparser::token::TokenStream;
use crate::token_types::TokenType;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input: input.chars().collect(), position: 0, line: 1, column: 1 }
    }

    fn scan_whitespace(&mut self) {
        while let Some(current_char) = self.current_char() {
            if current_char.is_whitespace() {
                self.advance_position();
            } else {
                break;
            }
        }
    }

    fn scan_line_comment(&mut self) {
        while let Some(ch) = self.current_char() {
            self.advance_position();
            if ch == '\n' {
                break;
            }
        }
    }

    fn scan_minus_or_arrow(&mut self, line: usize, column: usize) -> Token {
        let mut lexeme = String::from("-");

        if let Some(next_char) = self.current_char() {
            if next_char != '>' {
                return Token::new(TokenType::Minus, lexeme, line, column);
            } else {
                self.advance_position(); // Consume the '>'
                lexeme.push(next_char);
                return Token::new(TokenType::RightArrow, lexeme, line, column);
            }
        }
        Token::new(TokenType::Minus, lexeme, line, column)
    }

    fn scan_less_or_others(&mut self, line: usize, column: usize) -> Token {
        let mut lexeme = String::from("<");

        if let Some(next_char) = self.current_char() {
            return match next_char {
                '=' => {
                    self.advance_position(); // Consume the '='
                    lexeme.push(next_char);
                    Token::new(TokenType::LessThanOrEqual, lexeme, line, column)
                }
                '-' => {
                    self.advance_position(); // Consume the '-'
                    lexeme.push(next_char);
                    Token::new(TokenType::LeftArrow, lexeme, line, column)
                }
                '>' => {
                    self.advance_position(); // Consume the '>'
                    lexeme.push(next_char);
                    Token::new(TokenType::NotEqual, lexeme, line, column)
                }
                _ => {
                    Token::new(TokenType::LessThan, lexeme, line, column)
                }
            }
        }
        Token::new(TokenType::LessThan, lexeme, line, column)
    }

    fn scan_greater_or_others(&mut self, line: usize, column: usize) -> Token {
        let mut lexeme = String::from(">");

        if let Some(next_char) = self.current_char() {
            return match next_char {
                '=' => {
                    self.advance_position(); // Consume the '='
                    lexeme.push(next_char);
                    Token::new(TokenType::GreaterThanOrEqual, lexeme, line, column)
                }
                _ => {
                    Token::new(TokenType::GreaterThan, lexeme, line, column)
                }
            }
        }
        Token::new(TokenType::GreaterThan, lexeme, line, column)
    }

    fn scan_number(&mut self, first_digit: char, line: usize, column: usize) -> Token {
        let mut lexeme = String::from(first_digit);
        let mut has_comma = false;

        while let Some(current_char) = self.current_char() {
            if current_char.is_digit(10) {
                lexeme.push(current_char);
                self.advance_position();
            } else if current_char == ',' && !has_comma {
                has_comma = true;
                lexeme.push(current_char);
                self.advance_position();
            } else {
                break;
            }
        }

        let token_type = if has_comma {
            TokenType::Real
        } else {
            TokenType::Integer
        };

        Token::new(token_type, lexeme, line, column)
    }

    fn scan_equal_or_others(&mut self, line: usize, column: usize) -> Token {
        let mut lexeme = String::from("=");

        if let Some(next_char) = self.current_char() {
            return match next_char {
                '>' => {
                    self.advance_position(); // Consume the '>'
                    lexeme.push(next_char);
                    Token::new(TokenType::RightFatArrow, lexeme, line, column)
                }
                _ => {
                    Token::new(TokenType::Equal, lexeme, line, column)
                }
            }
        }
        Token::new(TokenType::Equal, lexeme, line, column)
    }

    fn scan_string(&mut self, line: usize, column: usize) -> Token {
        let mut lexeme = String::from('"');

        while let Some(current_char) = self.current_char() {
            lexeme.push(current_char);
            self.advance_position();
            if current_char == '"' {
                break;
            }
        }

        Token::new(TokenType::String, lexeme, line, column)
    }

    fn scan_identifier_or_keyword(&mut self, first_char: char, line: usize, column: usize) -> Token {
        let mut lexeme = String::from(first_char);

        while let Some(current_char) = self.current_char() {
            if current_char.is_alphanumeric() || current_char == '_' {
                lexeme.push(current_char);
                self.advance_position();
            } else {
                break;
            }
        }

        let token_type = TokenType::get_keyword_type(&lexeme).unwrap_or(TokenType::Identifier);
        Token::new(token_type, lexeme, line, column)
    }

    fn advance_position(&mut self) {
        if let Some(current_char) = self.current_char() {
            if current_char == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
}

impl TokenStream for Lexer {
    fn advance(&mut self) -> Option<Token> {
        loop {
            self.scan_whitespace();
            let ch = self.current_char();
            if ch.is_none() {
                break;
            }

            let ch = ch.unwrap();
            let line = self.line;
            let column = self.column;
            self.advance_position();

            match ch {
                ';' => {
                    self.scan_line_comment();
                    continue;
                }
                '+' => return Some(Token::new(TokenType::Plus, ch, line, column)),
                '-' => return Some(self.scan_minus_or_arrow(line, column)),
                '*' => return Some(Token::new(TokenType::Multiply, ch, line, column)),
                '/' => return Some(Token::new(TokenType::Divide, ch, line, column)),
                '^' => return Some(Token::new(TokenType::Caret, ch, line, column)),
                '(' => return Some(Token::new(TokenType::LeftParen, ch, line, column)),
                ')' => return Some(Token::new(TokenType::RightParen, ch, line, column)),
                '<' => return Some(self.scan_less_or_others(line, column)),
                '>' => return Some(self.scan_greater_or_others(line, column)),
                '=' => return Some(self.scan_equal_or_others(line, column)),
                '"' => return Some(self.scan_string(line, column)),
                _ => {},
            }

            if ch.is_digit(10) {
                return Some(self.scan_number(ch, line, column));
            }

            return Some(self.scan_identifier_or_keyword(ch, line, column));
        }

        None
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = r#"
        ; This is a line comment
        def factorial(n)
            begin
                def helper(i acc)
                    cond
                        i = 1 => acc
                        else => begin
                            def j i - 1
                            def new_acc i * acc
                            helper(j new_acc)
                        end
                    end
                helper(n 1)
            end
        "#;

        let mut lexer = Lexer::new(input.to_string());
        let mut tokens = Vec::new();

        while let Some(token) = lexer.advance() {
            tokens.push(token);
        }

        assert_eq!(tokens.len(), 44); // Adjust the expected number of tokens based on the input
        dbg!("{:?}", tokens);
    }
}