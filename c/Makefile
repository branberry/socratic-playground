CC      ?= cc
CFLAGS  ?= -Wall -Wextra -std=c11 -pedantic -I.
LDFLAGS ?=
BUILD   := build
BIN     := $(BUILD)/c-learn
TEST_BIN := $(BUILD)/run_tests

EXercise_SRC := \
	exercises/ex01_bytes.c \
	exercises/ex02_bits.c \
	exercises/ex03_file.c \
	exercises/ex04_cpu.c \
	exercises/ex05_fetch.c \
	exercises/ex06_step.c \
	exercises/ex08_run.c

TEST_SRC := \
	tests/run_tests.c \
	tests/ex01_test.c \
	tests/ex02_test.c \
	tests/ex03_test.c \
	tests/ex04_test.c \
	tests/ex05_test.c \
	tests/ex06_test.c \
	tests/ex07_test.c \
	tests/ex08_test.c

.PHONY: all build test clean fixtures

all: build

build: $(BIN)

$(BIN): src/main.c
	@mkdir -p $(BUILD)
	$(CC) $(CFLAGS) -o $@ $<

fixtures:
	@mkdir -p fixtures
	printf '\xde\xad\xbe\xef' > fixtures/ex03_tiny.bin
	printf '\x12\x05\x00\x01' > fixtures/ex08_hello.rom

test: fixtures $(TEST_BIN)
ifdef EX
	./$(TEST_BIN) $(EX)
else
	./$(TEST_BIN)
endif

$(TEST_BIN): $(EXercise_SRC) $(TEST_SRC) tests/harness.h
	@mkdir -p $(BUILD)
	$(CC) $(CFLAGS) -o $@ $(EXercise_SRC) $(TEST_SRC) $(LDFLAGS)

clean:
	rm -rf $(BUILD)
