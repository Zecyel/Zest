%{

%}

%token INTERFACE "interface"
%token FUNCTION "function"
%token STRUCT "struct"
%token IDENTIFER LITERAL

%%

interface
    : "interface" IDENTIFER '{' interface_content '}'

interface_content
    : %empty
    | interface_content interface_item

interface_item
    : function_head ';'

type
    : expression

function_head
    : "function" IDENTIFER '(' parameter_define_list ')' ':' type opt_scope

parameter_define_list
    : %empty
    | parameter_define_list ',' parameter_define_item

parameter_define_item
    : IDENTIFER ':' type opt_parameter_initial_value

opt_parameter_initial_value
    : %empty
    | '=' expression

function_call
    : IDENTIFER opt_generics '(' parameter_list ')'

opt_generics
    : %empty
    | '<' generic_content '>'

generic_content
    : generic_item
    | generic_content ',' generic_item

generic_item
    : type

parameter_list
    : %empty
    | parameter_list ',' parameter_item

parameter_item
    : expression

expression
    : IDENTIFER
    | LITERAL
    | expression '+' expression
    | function_call
%%
