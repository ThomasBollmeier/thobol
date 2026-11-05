use alteraparser::prelude::{Ast, ParseError, Parser as AltParser};
use crate::lexer::Lexer;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, input: &str) -> Result<Ast, ParseError> {
        let mut lexer = Lexer::new(input.to_string());
        let grammar = crate::grammar::create_grammar();
        let parser = AltParser::new(grammar);

        parser.parse(&mut lexer)
    }
}

mod tests {
    use super::*;
    use alteraparser::prelude::AstStrWriter;

    #[test]
    fn test_parse_simple_expression() {
        let code = "a + b";
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_function_definition() {
        let code = r#"
        def add(a b) a + b
        "#;
        parse_and_print_ast(code);
    }

    fn parse_and_print_ast(input: &str) {
        let parser = Parser::new();
        match parser.parse(input) {
            Ok(ast) => {
                let writer = AstStrWriter::new(2);
                let ast_str = writer.write_ast_to_str(&ast);
                println!("AST:\n{}", ast_str);
            }
            Err(e) => {
                println!("Parse error: {:?}", e);
            }
        }
    }

}