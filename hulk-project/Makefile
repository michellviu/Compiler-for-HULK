CC = gcc
CFLAGS = -Iinclude -Isrc/parser -lm
BIN = build/hulk-compiler

all: $(BIN)

$(BIN): build/lex.yy.o build/parser.tab.o src/main.o
	$(CC) $^ -o $@ $(CFLAGS)

build/lex.yy.o: src/lexer/lexer.l
	flex -o build/lex.yy.c $<
	$(CC) -c build/lex.yy.c -o $@ $(CFLAGS)

build/parser.tab.o: src/parser/parser.y
	bison -d -o build/parser.tab.c $<
	$(CC) -c build/parser.tab.c -o $@ $(CFLAGS)

src/main.o: src/main.c
	$(CC) -c $< -o $@ $(CFLAGS)

clean:
	rm -f build/* src/main.o

test: $(BIN)
	$(BIN) tests/test_programs/arithmetic.hulk
