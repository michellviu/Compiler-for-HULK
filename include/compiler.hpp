#ifndef COMPILER_H
#define COMPILER_H

#include <stdio.h>

extern FILE *yyin;
void yyerror(const char *s);
extern "C" int yyparse();
extern "C" int yylex();

#endif
