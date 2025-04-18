%{
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int yylex(void);
int yyparse(void);
void yyerror(const char *s);

extern int yylineno;

int execute = 1;
int error_flag = 0;
%}

%union {
    int ival;
}

%token IF ELSE WHILE PRINT ASSIGN LET IN
%token PLUS MINUS TIMES DIVIDE POWER SEMICOLON
%token LPAREN RPAREN LBRACE RBRACE
%token <ival> NUMBER

%type <ival> expr stmt stmt_list

%left PLUS MINUS
%left TIMES DIVIDE
%nonassoc UMINUS
%nonassoc POWER

%%

program:
    stmt_list
    ;

stmt_list:
    stmt
    | stmt_list stmt
    ;

stmt:
    expr SEMICOLON { if(!error_flag && execute) printf("Resultado: %d\n", $1);error_flag = 0; }
    // | PRINT expr SEMICOLON { if(execute) printf("Output: %d\n", $2); }
    // | IF LPAREN expr RPAREN stmt {
    //     int cond = $3;
    //     int old_exec = execute;
    //     execute = old_exec && cond;
    //     $$ = $5;
    //     execute = old_exec;
    // }
    // | IF LPAREN expr RPAREN stmt ELSE stmt {
    //     int cond = $3;
    //     int old_exec = execute;
        
    //     if(cond) {
    //         execute = old_exec;
    //         $$ = $5;
    //         execute = 0;
    //         (void)$7;
    //     } else {
    //         execute = 0;
    //         (void)$5;
    //         execute = old_exec;
    //         $$ = $7;
    //     }
    //     execute = old_exec;
    // }
    | LBRACE stmt_list RBRACE { if (!error_flag) $$ = $2; }
    ;

expr:
    NUMBER { $$ = $1; }
    | expr PLUS expr { $$ = $1 + $3; }
    | expr MINUS expr { $$ = $1 - $3; }
    | expr TIMES expr { $$ = $1 * $3; }
    | expr DIVIDE expr { 
        if($3 == 0) yyerror("Division por cero!");
        else $$ = $1 / $3;
    }
    | expr POWER expr { $$ = pow($1, $3); }
    | LPAREN expr RPAREN { $$ = $2; }
    ;

%%

void yyerror(const char *s) {
    fprintf(stderr, "Error en la linea %d: %s\n", yylineno, s);
    error_flag = 1;
}

