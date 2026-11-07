// Generated from grammar file: thobol.g

#![allow(unused)]
use alteraparser::prelude::*;

pub fn create_grammar() -> Grammar {
  let mut grammar = Grammar::new();
  
  grammar.add_rule("program", true, |g| {
    g.rule_ref("item_list")
  });
  
  grammar.add_rule("item_list", false, |g| {
    seq(vec![
      g.rule_ref_id("item", "it"),
      many(
        seq(vec![
          tok("SEMICOLON"),
          g.rule_ref_id("item", "it"),
        ])
      ),
      opt(
        tok_id("SEMICOLON", "final_semicolon")
      ),
    ])
  });
  
  grammar.add_rule("item", false, |g| {
    choice(vec![
      g.rule_ref("definition"),
      g.rule_ref("expression"),
    ])
  });
  
  grammar.add_rule("definition", false, |g| {
    choice(vec![
      g.rule_ref("var_definition"),
      g.rule_ref("fun_definition"),
      g.rule_ref("struct_definition"),
    ])
  });
  
  grammar.add_rule("var_definition", false, |g| {
    seq(vec![
      tok("DEF"),
      tok("IDENTIFIER"),
      g.rule_ref("expression"),
    ])
  });
  
  grammar.add_rule("fun_definition", false, |g| {
    seq(vec![
      tok("FUN"),
      tok("IDENTIFIER"),
      g.rule_ref("parameters"),
      g.rule_ref("expression"),
    ])
  });
  
  grammar.add_rule("parameters", false, |g| {
    seq(vec![
      tok("LEFT_PAREN"),
      choice(vec![
        seq(vec![
          tok_id("IDENTIFIER", "param"),
          many(
            seq(vec![
              tok("COMMA"),
              tok_id("IDENTIFIER", "param"),
            ])
          ),
          opt(
            seq(vec![
              tok("COMMA"),
              g.rule_ref_id("var_param", "var_param"),
            ])
          ),
        ]),
        opt(
          g.rule_ref_id("var_param", "var_param")
        ),
      ]),
      tok("RIGHT_PAREN"),
    ])
  });
  
  grammar.add_rule("var_param", false, |g| {
    seq(vec![
      tok("IDENTIFIER"),
      tok("DOT3"),
    ])
  });
  
  grammar.add_rule("struct_definition", false, |g| {
    seq(vec![
      tok("STRUCT"),
      tok_id("IDENTIFIER", "name"),
      tok("BEGIN"),
      choice(vec![
        tok_id("IDENTIFIER", "field"),
        g.rule_ref_id("fun_definition", "method"),
      ]),
      many(
        seq(vec![
          opt(
            tok("COMMA")
          ),
          choice(vec![
            tok_id("IDENTIFIER", "field"),
            g.rule_ref_id("fun_definition", "method"),
          ]),
        ])
      ),
      tok("END"),
    ])
  });
  
  grammar.add_rule("expression", false, |g| {
    choice(vec![
      g.rule_ref("block_expression"),
      g.rule_ref("assignment"),
    ])
  });
  
  grammar.add_rule("block_expression", false, |g| {
    seq(vec![
      tok("BEGIN"),
      g.rule_ref("item_list"),
      tok("END"),
    ])
  });
  
  grammar.add_rule("assignment", false, |g| {
    choice(vec![
      seq(vec![
        g.rule_ref_id("path_expression", "target"),
        tok("LEFT_ARROW"),
        g.rule_ref_id("comparison", "source"),
      ]),
      seq(vec![
        g.rule_ref_id("comparison", "source"),
        opt(
          seq(vec![
            tok("RIGHT_ARROW"),
            g.rule_ref_id("path_expression", "target"),
          ])
        ),
      ]),
    ])
  });
  
  grammar.add_rule("comparison", false, |g| {
    seq(vec![
      g.rule_ref_id("sum", "left"),
      opt(
        seq(vec![
          choice(vec![
            tok_id("EQUAL", "op"),
            tok_id("NOT_EQUAL", "op"),
            tok_id("LESS_THAN", "op"),
            tok_id("LESS_THAN_OR_EQUAL", "op"),
            tok_id("GREATER_THAN", "op"),
            tok_id("GREATER_THAN_OR_EQUAL", "op"),
          ]),
          g.rule_ref_id("sum", "right"),
        ])
      ),
    ])
  });
  
  grammar.add_rule("sum", false, |g| {
    seq(vec![
      g.rule_ref_id("product", "term"),
      many(
        seq(vec![
          choice(vec![
            tok_id("PLUS", "op"),
            tok_id("MINUS", "op"),
          ]),
          g.rule_ref_id("product", "term"),
        ])
      ),
    ])
  });
  
  grammar.add_rule("product", false, |g| {
    seq(vec![
      g.rule_ref_id("power", "fact"),
      many(
        seq(vec![
          choice(vec![
            tok_id("MULTIPLY", "op"),
            tok_id("DIVIDE", "op"),
          ]),
          g.rule_ref_id("power", "fact"),
        ])
      ),
    ])
  });
  
  grammar.add_rule("power", false, |g| {
    seq(vec![
      g.rule_ref_id("primary", "base"),
      opt(
        seq(vec![
          tok_id("CARET", "op"),
          g.rule_ref_id("power", "exponent"),
        ])
      ),
    ])
  });
  
  grammar.add_rule("primary", false, |g| {
    choice(vec![
      seq(vec![
        tok("LEFT_PAREN"),
        g.rule_ref("assignment"),
        tok("RIGHT_PAREN"),
      ]),
      g.rule_ref("if_expression"),
      g.rule_ref("cond_expression"),
      g.rule_ref("lambda_expression"),
      g.rule_ref("atomic"),
    ])
  });
  
  grammar.add_rule("if_expression", false, |g| {
    seq(vec![
      tok("IF"),
      g.rule_ref_id("expression", "condition"),
      g.rule_ref_id("expression", "then_branch"),
      opt(
        seq(vec![
          tok("ELSE"),
          g.rule_ref_id("expression", "else_branch"),
        ])
      ),
      tok("END"),
    ])
  });
  
  grammar.add_rule("cond_expression", false, |g| {
    seq(vec![
      tok("COND"),
      many(
        g.rule_ref("cond_arm")
      ),
      opt(
        g.rule_ref("default_arm")
      ),
      tok("END"),
    ])
  });
  
  grammar.add_rule("cond_arm", false, |g| {
    seq(vec![
      g.rule_ref_id("expression", "condition"),
      tok("RIGHT_FAT_ARROW"),
      g.rule_ref_id("expression", "result"),
    ])
  });
  
  grammar.add_rule("default_arm", false, |g| {
    seq(vec![
      tok("ELSE"),
      g.rule_ref_id("expression", "result"),
    ])
  });
  
  grammar.add_rule("lambda_expression", false, |g| {
    seq(vec![
      tok("LAMBDA"),
      g.rule_ref("parameters"),
      g.rule_ref("expression"),
    ])
  });
  
  grammar.add_rule("atomic", false, |g| {
    choice(vec![
      tok("INTEGER"),
      tok("REAL"),
      tok("BOOL"),
      tok("STRING"),
      tok("NIL"),
      g.rule_ref("path_expression"),
    ])
  });
  
  grammar.add_rule("path_expression", false, |g| {
    seq(vec![
      g.rule_ref_id("path_segment", "segment"),
      many(
        seq(vec![
          tok("DOT"),
          g.rule_ref_id("path_segment", "segment"),
        ])
      ),
    ])
  });
  
  grammar.add_rule("path_segment", false, |g| {
    seq(vec![
      tok_id("IDENTIFIER", "id"),
      many(
        g.rule_ref_id("arguments", "args")
      ),
    ])
  });
  
  grammar.add_rule("arguments", false, |g| {
    seq(vec![
      tok("LEFT_PAREN"),
      choice(vec![
        seq(vec![
          g.rule_ref_id("comparison", "argument"),
          many(
            seq(vec![
              tok("COMMA"),
              g.rule_ref_id("comparison", "argument"),
            ])
          ),
          opt(
            seq(vec![
              tok("COMMA"),
              tok("DOT3"),
              g.rule_ref_id("comparison", "vararg"),
            ])
          ),
        ]),
        opt(
          seq(vec![
            tok("DOT3"),
            g.rule_ref_id("comparison", "vararg"),
          ])
        ),
      ]),
      tok("RIGHT_PAREN"),
    ])
  });
  
  grammar
}
