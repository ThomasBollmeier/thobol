@start
program -> (definition | expression)*;

definition -> var_definition | fun_definition;

var_definition -> DEF IDENTIFIER expression;

fun_definition -> FUN IDENTIFIER parameters expression;

parameters -> LEFT_SQUARE_BRACKET (param#IDENTIFIER (COMMA? param#IDENTIFIER)*)? RIGHT_SQUARE_BRACKET;

expression -> block_expression | assignment;

block_expression -> BEGIN item#definition* item#expression+ END;

assignment -> target#IDENTIFIER LEFT_ARROW source#comparison |
    source#comparison (RIGHT_ARROW target#IDENTIFIER)?;

comparison ->
    left#sum
    ((op#EQUAL | op#NOT_EQUAL | op#LESS_THAN | op#LESS_THAN_OR_EQUAL | op#GREATER_THAN | op#GREATER_THAN_OR_EQUAL)
    right#sum)?;

sum -> term#product ((op#PLUS | op#MINUS) term#product)*;

product -> fact#power ((op#MULTIPLY | op#DIVIDE) fact#power)*;

power -> base#primary (op#CARET exponent#power)?;

primary -> LEFT_PAREN assignment RIGHT_PAREN | if_expression | cond_expression
    | lambda_expression | atomic;

if_expression -> IF condition#expression then_branch#expression (ELSE else_branch#expression)? END;

cond_expression -> COND cond_arm* default_arm? END;

cond_arm -> condition#expression RIGHT_FAT_ARROW result#expression;

default_arm -> ELSE result#expression;

lambda_expression -> LAMBDA parameters expression;

atomic -> INTEGER | REAL | BOOL | STRING | NIL |
    IDENTIFIER args#arguments*;

arguments -> LEFT_SQUARE_BRACKET (argument#comparison (COMMA? argument#comparison)*)? RIGHT_SQUARE_BRACKET;
