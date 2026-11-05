@start
program -> (definition | expression )*;

definition -> var_definition;

var_definition -> DEF name#IDENTIFIER (LEFT_PAREN param#IDENTIFIER* RIGHT_PAREN)? body#expression;

expression -> sum;

sum -> term#term ((op#PLUS| op#MINUS) term#term)*;

term -> atomic;

atomic -> INTEGER | REAL | BOOL | STRING | IDENTIFIER;
