# Variables
CXX = g++ # Compilador de C++ para el ejecutable final
CC = gcc  # Compilador de C para los archivos de Flex y Bison
CXXFLAGS = -Iinclude -Isrc/parser -Ibuild -lm # Bandera para C++
CFLAGS = -Iinclude -Isrc/parser -Ibuild -lm   # Bandera para C
BIN = build/hulk-compiler
SCRIPT = script.hulk

# Detectar sistema operativo
ifeq ($(OS),Windows_NT)
    MKDIR = mkdir
    CHECK_FILE = if not exist
    RM = rmdir /S /Q
else
    MKDIR = mkdir -p
    CHECK_FILE = test -f
    RM = rm -rf
endif

# Regla principal
all: build

# Regla build
build: $(BIN)
	@if [ -f "$(SCRIPT)" ]; then echo "[OK] Archivo 'script.hulk' encontrado."; else echo "[ERROR] Archivo 'script.hulk' no encontrado." && exit 1; fi
	@echo "[OK] Build completado."

# Generar el ejecutable
$(BIN): build/parser.tab.o build/lex.yy.o src/main.o | build-dir
	$(CXX) build/parser.tab.o build/lex.yy.o src/main.o -o $@ $(CXXFLAGS)

# Crear el directorio build si no existe
build-dir:
	@$(MKDIR) build

# Generar archivos de Bison
build/parser.tab.c build/parser.tab.h: src/parser/parser.y | build-dir
	bison -d -o build/parser.tab.c $<

# Generar archivo de Flex
build/lex.yy.c: src/lexer/lexer.l build/parser.tab.h | build-dir
	flex -o $@ $<

# Compilar archivos objeto
build/lex.yy.o: build/lex.yy.c | build-dir
	$(CC) -c $< -o $@ $(CFLAGS)

build/parser.tab.o: build/parser.tab.c | build-dir
	$(CC) -c $< -o $@ $(CFLAGS)

src/main.o: src/main.cpp | build-dir
	$(CXX) -c $< -o $@ $(CXXFLAGS)

# Regla run
run: build
	@echo "[INFO] Ejecutando el compilador..."
	@$(BIN) $(SCRIPT)

# Regla clean
clean:
	@rm -rf build
	@echo "[OK] Operación clean completada."

.PHONY: all build run clean
