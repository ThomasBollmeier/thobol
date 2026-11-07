use crate::lexer::Lexer;
use alteraparser::prelude::{Ast, Grammar, ParseError, Parser as AltParser};

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, input: &str) -> Result<Ast, ParseError> {
        let mut lexer = Lexer::new(input.to_string());

        let mut grammar = crate::grammar::create_grammar();
        Self::add_ast_transformers(&mut grammar);

        let parser = AltParser::new(grammar);

        parser.parse(&mut lexer)
    }

    fn add_ast_transformers(grammar: &mut Grammar) {
        grammar.add_ast_transformer("program", Self::transform_program);
        grammar.add_ast_transformer("item_list", Self::transform_item_list);
        grammar.add_ast_transformer("item", Self::single_child);
        grammar.add_ast_transformer("definition", Self::single_child);
        grammar.add_ast_transformer("var_definition", Self::transform_var_definition);
        grammar.add_ast_transformer("fun_definition", Self::transform_fun_definition);
        grammar.add_ast_transformer("struct_definition", Self::transform_struct_definition);
        grammar.add_ast_transformer("parameters", Self::transform_parameters);
        grammar.add_ast_transformer("var_param", Self::transform_var_param);
        grammar.add_ast_transformer("expression", Self::single_child);
        grammar.add_ast_transformer("block_expression", Self::transform_block_expression);
        grammar.add_ast_transformer("assignment", Self::transform_assignment);
        grammar.add_ast_transformer("comparison", Self::transform_comparison);
        grammar.add_ast_transformer("sum", |ast| Self::transform_binary(ast, "term", "op"));
        grammar.add_ast_transformer("product", |ast| Self::transform_binary(ast, "fact", "op"));
        grammar.add_ast_transformer("power", Self::transform_power);
        grammar.add_ast_transformer("primary", Self::transform_primary);
        grammar.add_ast_transformer("if_expression", Self::transform_if_expression);
        grammar.add_ast_transformer("cond_expression", Self::transform_cond_expression);
        grammar.add_ast_transformer("cond_arm", Self::transform_cond_arm);
        grammar.add_ast_transformer("default_arm", Self::transform_default_arm);
        grammar.add_ast_transformer("lambda_expression", Self::transform_lambda_expression);
        grammar.add_ast_transformer("atomic", Self::single_child);
        grammar.add_ast_transformer("path_expression", Self::transform_path_expression);
        grammar.add_ast_transformer("path_segment", Self::transform_path_segment);
        grammar.add_ast_transformer("arguments", Self::transform_arguments);
    }

    fn transform_program(ast: Ast) -> Ast {
        let mut ret = Ast::new("program");
        let children = ast.children();
        let item_list = &children[0];
        for child in item_list.children() {
            ret.add_child(child.clone());
        }
        ret
    }

    fn transform_item_list(ast: Ast) -> Ast {
        let mut ret = Ast::new("item_list");
        for child in ast.children_by_id("it", true) {
            ret.add_child(child);
        }
        if let Some(_) = ast.child_by_id("final_semicolon", true) {
            ret.add_child(Ast::with_value("NIL", "nil"));
        }
        ret
    }

    fn transform_var_definition(ast: Ast) -> Ast {
        let children = ast.children();
        let name = children[1]
            .clone()
            .value
            .expect("Missing value in name of var_definition");
        let value = children[2].clone();

        let mut ret = Ast::new("var_definition");
        ret.add_child(Ast::with_value("name", name));
        ret.add_child(value);

        ret
    }

    fn transform_fun_definition(ast: Ast) -> Ast {
        let children = ast.children();
        let name = children[1]
            .clone()
            .value
            .expect("Missing value in name of fun_definition");
        let parameters = children[2].clone();
        let body = children[3].clone();

        let mut ret = Ast::new("var_definition");
        ret.add_child(Ast::with_value("name", name));
        let mut lambda_expression = Ast::new("lambda_expression");
        lambda_expression.add_child(parameters);
        lambda_expression.add_child(body);
        ret.add_child(lambda_expression);

        ret
    }

    fn transform_struct_definition(ast: Ast) -> Ast {
        let name = ast
            .child_by_id("name", true)
            .expect("Missing name in struct_definition")
            .value
            .expect("Missing value in name of struct_definition");
        let mut ret = Ast::new("struct_definition");
        ret.add_child(Ast::with_value("name", name));

        let mut fields = Ast::new("fields");
        for field in ast.children_by_id("field", true) {
            fields.add_child(field);
        }
        ret.add_child(fields);

        let mut methods = Ast::new("methods");
        for method in ast.children_by_id("method", true) {
            methods.add_child(method);
        }
        ret.add_child(methods);

        ret
    }

    fn transform_parameters(ast: Ast) -> Ast {
        let mut ret = Ast::new("parameters");
        for child in ast.children_by_id("param", true) {
            let param_name = child.value.expect("Missing value in param of parameters");
            ret.add_child(Ast::with_value("parameter", param_name));
        }

        if let Some(var_param) = ast.child_by_id("var_param", true) {
            ret.add_child(var_param);
        }

        ret
    }

    fn transform_var_param(ast: Ast) -> Ast {
        let children = ast.children();
        let param_name = children[0]
            .clone()
            .value
            .expect("Missing value in var_param");
        Ast::with_value("var_parameter", param_name)
    }

    fn transform_block_expression(ast: Ast) -> Ast {
        let mut ret = Ast::new("block_expression");
        let item_list = ast
            .children()
            .get(1)
            .expect("Missing item_list in block_expression");
        for child in item_list.children() {
            ret.add_child(child.clone());
        }
        ret
    }

    fn transform_assignment(ast: Ast) -> Ast {
        let source = ast
            .child_by_id("source", true)
            .expect("Missing source in assignment");
        match ast.child_by_id("target", true) {
            Some(target) => {
                let mut ret = Ast::new("assignment");
                ret.add_child(source);
                ret.add_child(target);
                ret
            }
            None => source,
        }
    }

    fn transform_comparison(ast: Ast) -> Ast {
        let left = ast
            .child_by_id("left", true)
            .expect("Missing left in comparison");
        match ast.child_by_id("right", true) {
            Some(right) => {
                let operator = ast
                    .child_by_id("op", true)
                    .expect("Missing operator in comparison")
                    .value
                    .expect("Missing value in operator of comparison");
                let mut ret = Ast::with_value("comparison", operator);
                ret.add_child(left);
                ret.add_child(right);
                ret
            }
            None => left,
        }
    }

    fn transform_binary(ast: Ast, operand_id: &str, operator_id: &str) -> Ast {
        let operands = ast.children_by_id(operand_id, true);
        let operators = ast.children_by_id(operator_id, true);

        let mut ret = operands[0].clone();
        for (i, op) in operators.iter().enumerate() {
            let op_name = match op.name.as_str() {
                "PLUS" => "add",
                "MINUS" => "subtract",
                "MULTIPLY" => "multiply",
                "DIVIDE" => "divide",
                _ => panic!("Unexpected operator in sum: {}", op.name),
            };
            let mut new_node = Ast::new(op_name);
            new_node.add_child(ret);
            new_node.add_child(operands[i + 1].clone());
            ret = new_node;
        }

        ret
    }

    fn transform_power(ast: Ast) -> Ast {
        let base = ast
            .child_by_id("base", true)
            .expect("Missing base in power");
        match ast.child_by_id("exponent", true) {
            Some(exponent) => {
                let mut ret = Ast::new("power");
                ret.add_child(base);
                ret.add_child(exponent);
                ret
            }
            None => base,
        }
    }

    fn transform_primary(ast: Ast) -> Ast {
        let children = ast.children();
        if children.len() == 1 {
            return children[0].clone();
        } else if children.len() == 3 {
            // This is a parenthesized expression: ( assignment )
            return children[1].clone();
        }
        panic!(
            "Unexpected number of children in primary: {}",
            children.len()
        );
    }

    fn transform_if_expression(ast: Ast) -> Ast {
        let condition = ast
            .child_by_id("condition", true)
            .expect("Missing condition in if_expression");
        let then_branch = ast
            .child_by_id("then_branch", true)
            .expect("Missing then_branch in if_expression");
        let else_branch = ast.child_by_id("else_branch", true);

        let mut ret = Ast::new("if_expression");
        ret.add_child(condition);
        ret.add_child(then_branch);
        if let Some(else_branch) = else_branch {
            ret.add_child(else_branch);
        }
        ret
    }

    fn transform_cond_expression(ast: Ast) -> Ast {
        let mut ret = Ast::new("cond_expression");
        for child in ast.children() {
            match child.name.as_str() {
                "COND" | "END" => {}
                _ => {
                    ret.add_child(child.clone());
                }
            }
        }
        ret
    }

    fn transform_cond_arm(ast: Ast) -> Ast {
        let condition = ast
            .child_by_id("condition", true)
            .expect("Missing condition in cond_arm");
        let result = ast
            .child_by_id("result", true)
            .expect("Missing result in cond_arm");

        let mut ret = Ast::new("cond_arm");
        ret.add_child(condition);
        ret.add_child(result);
        ret
    }

    fn transform_lambda_expression(ast: Ast) -> Ast {
        let children = ast.children();
        let parameters = children[1].clone();
        let body = children[2].clone();

        let mut ret = Ast::new("lambda_expression");
        ret.add_child(parameters);
        ret.add_child(body);
        ret
    }

    fn transform_default_arm(ast: Ast) -> Ast {
        let default = ast
            .child_by_id("result", true)
            .expect("Missing result in default_arm");

        let mut ret = Ast::new("default_arm");
        ret.add_child(default);
        ret
    }

    fn transform_path_expression(ast: Ast) -> Ast {
        let segments = ast.children_by_id("segment", true);
        if segments.len() == 1 {
            return segments[0].clone();
        }
        let mut ret = Ast::new("path_expression");
        for segment in segments {
            ret.add_child(segment);
        }
        ret
    }

    fn transform_path_segment(ast: Ast) -> Ast {
        let identifier = ast
            .child_by_id("id", true)
            .expect("Missing id in path_segment");
        let args_list = ast.children_by_id("args", true);

        if args_list.is_empty() {
            return identifier;
        }

        let mut ret = Ast::new("call");
        ret.add_child(identifier);
        ret.add_child(args_list[0].clone());

        for args in args_list.iter().skip(1) {
            let mut new_call = Ast::new("call");
            new_call.add_child(ret);
            new_call.add_child(args.clone());
            ret = new_call;
        }

        ret
    }

    fn transform_arguments(ast: Ast) -> Ast {
        let mut ret = Ast::new("arguments");
        for child in ast.children_by_id("argument", true) {
            ret.add_child(child.clone());
        }
        if let Some(vararg) = ast.child_by_id("vararg", true) {
            let mut spread_node = Ast::new("spread");
            spread_node.add_child(vararg);
            ret.add_child(spread_node);
        }
        ret
    }

    fn single_child(ast: Ast) -> Ast {
        ast.children()[0].clone()
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
        fun fibonacci(n)
        begin
            fun helper(a, b, i)
                if i < n
                    helper(b, a+b, i+1)
                else
                    a
                end;
            helper(1, 1, 0)
        end
        "#;
        parse_and_print_ast(code);
    }

    #[test]
    fn test_fun_def_with_var_param() {
        let code = r#"
        fun sum_all(a, b, rest...)
            begin
                def total a + b;
                total <- total + rest.sum();
                total
            end;
        sum_all(1, 2, 3.14, 4, 5) -> result
        "#;
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_struct_definition() {
        let code = r#"
        struct Point
        begin
          x, y

          fun init(x, y)
            begin
              ego.x <- x;
              ego.y <- y;
            end

          fun move(dx, dy)
            Point(x + dx, y + dy)

        end
        "#;
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_assignment() {
        let code = "answer <- 42";
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_assignment_with_right_arrow() {
        let code = "x + 1 -> x";
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_if_expression() {
        let code = r#"
        if true 1 else 0 end
        "#;
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_nested_if_expression() {
        let code = r#"
        if true
            if false
                1
            end
        else
            2
        end
        "#;
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_cond_expression() {
        let code = r#"
        cond
            true => 1
            else 0
        end
        "#;
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_numeric_expression() {
        let code = "3 * (4 + 5)";
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_power_expression() {
        let code = "2 ^ 3 ^ 4";
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_lambda_expression() {
        let code = r#"
        map(λ(x) (x * x), xs) -> squares
        "#;
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_calls_of_calls() {
        let code = r#"
        fun make_greeter(before, after)
            lambda (name) (before + name + after);
        make_greeter("Hello, ", "!")("World") -> greeting
        "#;
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_path_expression() {
        let code = r#"
        my_struct.my_method(42).another_method("hello") -> result
        "#;
        parse_and_print_ast(code);
    }

    #[test]
    fn test_parse_spread_arguments() {
        let code = r#"
        fun sum(a, b) a + b;
        def numbers Array(1, 2);
        sum(...numbers) -> result
        "#;
        parse_and_print_ast(code);
    }

    #[allow(unused)]
    fn parse_and_print_ast(code: &str) {
        let parser = Parser::new();
        match parser.parse(code) {
            Ok(ast) => {
                let writer = AstStrWriter::new(2);
                let ast_str = writer.write_ast_to_str(&ast);
                println!("AST:\n{}", ast_str);
            }
            Err(e) => {
                panic!("Parse error: {:?}", e);
            }
        }
    }
}
