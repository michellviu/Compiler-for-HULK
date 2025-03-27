#include "parser/parser.h"
#include <iostream>

int main() {
    std::cout << "Calculadora HULK\n";
    std::cout << "Ingrese expresiones (ej: 2+3*4)\n";
    yyparse();
    return 0;
}