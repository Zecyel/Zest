%{
    #include "parser.tab.c"
%}

punctual [`~!@#$%\^&*()-=_+\[\]\\{}|;:,./<>?]

%%

interface { return INTERFACE; }
struct { return STRUCT; }
function { return FUNCTION; }
 // many other keywords omitted.

{punctual} { return yytext[0]; }

[a-zA-Z_][a-zA-Z0-9_]* {
    yylval = alloc_identifier(yytext);
    return IDENTIFER;
}

 // literal needed.