#include <stdio.h>
#include <stdlib.h>
#include "compiler.h"

extern FILE *yyin;

int main(int argc, char *argv[]) {
    if (argc > 1) {
        FILE *file = fopen(argv[1], "r");
        if (!file) {
            perror("Error al abrir archivo");
            return 1;
        }
        yyin = file;
    }
    
    yyparse();
    return 0;
}
