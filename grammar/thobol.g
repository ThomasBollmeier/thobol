@start
program -> item_list;

item_list -> it#item (SEMICOLON it#item)* final_semicolon#SEMICOLON?;

item -> definition | expression;

definition -> var_definition | fun_definition | struct_definition;

var_definition -> DEF IDENTIFIER expression;

fun_definition -> FUN IDENTIFIER parameters expression;

parameters -> LEFT_PAREN
    (param#IDENTIFIER (COMMA? param#IDENTIFIER)* (COMMA? var_param#var_param)? |
     var_param#var_param?)
    RIGHT_PAREN;

var_param -> IDENTIFIER DOT3;

struct_definition -> STRUCT name#IDENTIFIER BEGIN
    (field#IDENTIFIER | method#fun_definition)
    (COMMA? (field#IDENTIFIER | method#fun_definition))*
    END;

expression -> block_expression | assignment;

block_expression -> BEGIN item_list END;

assignment -> target#path_expression LEFT_ARROW source#comparison |
    source#comparison (RIGHT_ARROW target#path_expression)?;

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

atomic -> INTEGER | REAL | BOOL | STRING | NIL | path_expression;

path_expression -> segment#path_segment (DOT segment#path_segment)*;

path_segment -> id#IDENTIFIER args#arguments*;

arguments -> LEFT_PAREN
    (argument#comparison (COMMA? argument#comparison)* (COMMA? DOT3 vararg#comparison)? |
     (DOT3 vararg#comparison)?)
    RIGHT_PAREN;
