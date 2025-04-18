%{
#include <stdio.h>
#include "parser.tab.h"

extern int yylineno;
void yyerror(const char *s);
%}

%option noyywrap

DIGIT   [0-9]
ID      [a-zA-Z][a-zA-Z0-9]*

%%

"if"        return IF;
"else"      return ELSE;
"while"     return WHILE;
"print"     return PRINT;
"let"       return LET;
"in"        return IN;

"+"         return PLUS;
"-"         return MINUS;
"*"         return TIMES;
"/"         return DIVIDE;
"^"         return POWER;
"="         return ASSIGN;
";"         return SEMICOLON;

"{"         return LBRACE;
"}"         return RBRACE;
"("         return LPAREN;
")"         return RPAREN;

{DIGIT}+    { yylval.ival = atoi(yytext); return NUMBER; }

\n  { yylineno++; }
[ \t\r]+    { /* ignorar espacios */ }

.           { yyerror("Caracter desconocido"); }

%%
