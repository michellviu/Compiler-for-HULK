%{
#include <stdio.h>
#include <stdlib.h>
#include "compiler.h"

extern int yylex(void);
extern FILE *yyin;
extern int yyparse(void);

int execute = 1;
%}

%union {
    int ival;
}

%token IF ELSE WHILE PRINT ASSIGN
%token PLUS MINUS TIMES DIVIDE SEMICOLON
%token LPAREN RPAREN LBRACE RBRACE
%token <ival> NUMBER

%type <ival> expr stmt stmt_list

%left PLUS MINUS
%left TIMES DIVIDE
%nonassoc UMINUS

%%

program:
    stmt_list
    ;

stmt_list:
    stmt
    | stmt_list stmt
    ;

stmt:
    expr SEMICOLON { if(execute) printf("Resultado: %d\n", $1); }
    | PRINT expr SEMICOLON { if(execute) printf("Output: %d\n", $2); }
    | IF LPAREN expr RPAREN stmt {
        int cond = $3;
        int old_exec = execute;
        execute = old_exec && cond;
        $$ = $5;
        execute = old_exec;
    }
    | IF LPAREN expr RPAREN stmt ELSE stmt {
        int cond = $3;
        int old_exec = execute;
        
        if(cond) {
            execute = old_exec;
            $$ = $5;
            execute = 0;
            (void)$7;
        } else {
            execute = 0;
            (void)$5;
            execute = old_exec;
            $$ = $7;
        }
        execute = old_exec;
    }
    | LBRACE stmt_list RBRACE { $$ = $2; }
    ;

expr:
    NUMBER { $$ = $1; }
    | expr PLUS expr { $$ = $1 + $3; }
    | expr MINUS expr { $$ = $1 - $3; }
    | expr TIMES expr { $$ = $1 * $3; }
    | expr DIVIDE expr { 
        if($3 == 0) yyerror("Error: División por cero!");
        else $$ = $1 / $3;
    }
    | LPAREN expr RPAREN { $$ = $2; }
    ;

%%

void yyerror(const char *s) {
    fprintf(stderr, "Error: %s\n", s);
}

