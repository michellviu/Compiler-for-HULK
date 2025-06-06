# Makefile para Hulk Compiler

ifeq ($(OS),Windows_NT)
    RM = del /Q
    EXE = .exe
    RUN = .\script.exe
else
    RM = rm -rf
    EXE =
    RUN = ./script
endif

CLANG = clang
SCRIPT = script.hulk
BUILD_DIR = build

.PHONY: build run clean

build:
	@if [ ! -f $(SCRIPT) ]; then echo "ERROR: Falta $(SCRIPT) en el directorio actual." && exit 1; fi
	@mkdir -p $(BUILD_DIR)
	@echo "Compilando script.hulk..."
	@cargo run -- $(SCRIPT)

run: build
	@echo "Ejecutando compilador Hulk..."
	@echo "Generando ejecutable con clang..."
	@$(CLANG) build/script.ll -o build/script$(EXE)
	@$(if $(findstring Windows_NT,$(OS)), \
		$(BUILD_DIR)\script.exe $(SCRIPT), \
		$(BUILD_DIR)/script $(SCRIPT))

clean:
	@-rm -rf $(BUILD_DIR)