%{
#include <stdio.h>
#include "parser.tab.h"
#include "compiler.h"

extern FILE *yyin;
void yyerror(const char *s);
%}

%option noyywrap

DIGIT   [0-9]
ID      [a-zA-Z_][a-zA-Z0-9_]*

%%

"if"        return IF;
"else"      return ELSE;
"while"     return WHILE;
"print"     return PRINT;

"+"         return PLUS;
"-"         return MINUS;
"*"         return TIMES;
"/"         return DIVIDE;
"="         return ASSIGN;
";"         return SEMICOLON;

"{"         return LBRACE;
"}"         return RBRACE;
"("         return LPAREN;
")"         return RPAREN;

{DIGIT}+    { yylval.ival = atoi(yytext); return NUMBER; }

[ \t\n]+    { /* ignorar espacios */ }
.           { yyerror("Caracter desconocido"); }

%%
