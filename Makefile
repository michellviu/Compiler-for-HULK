CC = gcc
CFLAGS = -Iinclude -Isrc/parser -Ibuild -lm
BIN = build/hulk-compiler

all: $(BIN)

$(BIN): build/parser.tab.o build/lex.yy.o src/main.o
	$(CC) $^ -o $@ $(CFLAGS)

build/parser.tab.c build/parser.tab.h: src/parser/parser.y
	bison -d -o build/parser.tab.c $<

build/lex.yy.c: src/lexer/lexer.l build/parser.tab.h
	flex -o $@ $<

build/lex.yy.o: build/lex.yy.c
	$(CC) -c $< -o $@ $(CFLAGS)

build/parser.tab.o: build/parser.tab.c
	$(CC) -c $< -o $@ $(CFLAGS)

src/main.o: src/main.c
	$(CC) -c $< -o $@ $(CFLAGS)

clean:
	rm -f build/* src/main.o

test: $(BIN)
	$(BIN) tests/test_programs/arithmetic.hulk

.PHONY: all clean test
