#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    Integer,
    Real,
    Bool,
    String,
    Nil,
    Plus,
    Minus,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Semicolon,
    Dot,
    Dot3,
    Identifier,
    Begin,
    End,
    If,
    Else,
    Cond,
    Def,
    Fun,
    Lambda,
    Struct,
    RightArrow,
    RightFatArrow,
    LeftArrow,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Not,
    Error,
}

impl TokenType {
    pub fn get_keyword_type(keyword: &str) -> Option<TokenType> {
        match keyword {
            "begin" => Some(TokenType::Begin),
            "end" => Some(TokenType::End),
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "cond" => Some(TokenType::Cond),
            "def" => Some(TokenType::Def),
            "fun" => Some(TokenType::Fun),
            "lambda" | "λ" => Some(TokenType::Lambda),
            "struct" => Some(TokenType::Struct),
            "and" => Some(TokenType::And),
            "or" => Some(TokenType::Or),
            "not" => Some(TokenType::Not),
            "true" | "false" => Some(TokenType::Bool),
            "nil" => Some(TokenType::Nil),
            _ => None,
        }
    }

    fn camel_case_to_snake_upper_case(s: &str) -> String {
        let mut snake_case = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() && i != 0 {
                snake_case.push('_');
            }
            snake_case.push(c.to_ascii_uppercase());
        }
        snake_case
    }
}

impl Into<String> for TokenType {
    fn into(self) -> String {
        Self::camel_case_to_snake_upper_case(&format!("{:?}", self))
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_token_type_to_string() {
        let token_type = TokenType::LeftParen;
        let token_type_string: String = token_type.into();
        assert_eq!(token_type_string, "LEFT_PAREN");
    }
}
