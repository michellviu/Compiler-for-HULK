#include <iostream>
#include <fstream>
#include "compiler.hpp"

extern FILE *yyin;
extern "C" int yyparse();
extern "C" int yylex();

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Uso: %s <archivo>\n", argv[0]);
        return 1;
    }

    // Abrir el archivo de entrada
    yyin = fopen(argv[1], "r");
    if (!yyin) {
        perror("Error al abrir el archivo");
        return 1;
    }

    // Ejecutar el análisis sintáctico
    if (yyparse() == 0) {
        printf("Analisis completado con exito.\n");
    } else {
        printf("Se encontraron errores durante el análisis.\n");
    }

    fclose(yyin);
    return 0;
}