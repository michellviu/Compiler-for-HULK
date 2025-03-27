#ifndef COMPILER_H
#define COMPILER_H

#include <stdio.h>

extern FILE *yyin;
void yyerror(const char *s);
int yyparse(void);
int yylex(void);

#endif
