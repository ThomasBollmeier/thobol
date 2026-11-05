// Generated from grammar file: thobol.g

#![allow(unused)]
use alteraparser::prelude::*;

pub fn create_grammar() -> Grammar {
  let mut grammar = Grammar::new();
  
  grammar.add_rule("program", true, |g| {
    many(
      choice(vec![
        g.rule_ref("definition"),
        g.rule_ref("expression"),
      ])
    )
  });
  
  grammar.add_rule("definition", false, |g| {
    g.rule_ref("var_definition")
  });
  
  grammar.add_rule("var_definition", false, |g| {
    seq(vec![
      tok("DEF"),
      tok_id("IDENTIFIER", "name"),
      opt(
        seq(vec![
          tok("LEFT_PAREN"),
          many(
            tok_id("IDENTIFIER", "param")
          ),
          tok("RIGHT_PAREN"),
        ])
      ),
      g.rule_ref_id("expression", "body"),
    ])
  });
  
  grammar.add_rule("expression", false, |g| {
    g.rule_ref("sum")
  });
  
  grammar.add_rule("sum", false, |g| {
    seq(vec![
      g.rule_ref_id("term", "term"),
      many(
        seq(vec![
          choice(vec![
            tok_id("PLUS", "op"),
            tok_id("MINUS", "op"),
          ]),
          g.rule_ref_id("term", "term"),
        ])
      ),
    ])
  });
  
  grammar.add_rule("term", false, |g| {
    g.rule_ref("atomic")
  });
  
  grammar.add_rule("atomic", false, |g| {
    choice(vec![
      tok("INTEGER"),
      tok("REAL"),
      tok("BOOL"),
      tok("STRING"),
      tok("IDENTIFIER"),
    ])
  });
  
  grammar
}
