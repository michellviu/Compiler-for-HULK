# Configuración para Windows/Linux
ifeq ($(OS),Windows_NT)
    MKDIR = mkdir
    RMDIR = rmdir /Q /S
    SCRIPT = script.hulk
    EXE = hulk-compiler.exe
else
    MKDIR = mkdir -p
    RMDIR = rm -rf
    SCRIPT = script.hulk
    EXE = hulk-compiler
endif

.PHONY: build run clean

build:
	@$(MKDIR) build
	@if ! [ -f "$(SCRIPT)" ]; then echo "[ERROR] Archivo $(SCRIPT) no encontrado" && exit 1; fi
	@echo "[OK] Build completado. Carpeta 'build' creada."

run: build
	@echo "[INFO] Ejecución pendiente (implementar luego)."

clean:
	@$(RMDIR) build
	@echo "[OK] Carpeta 'build' eliminada."
