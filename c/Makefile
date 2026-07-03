CC      ?= cc
CFLAGS  ?= -Wall -Wextra -std=c11 -pedantic
BUILD   := build
BIN     := $(BUILD)/c-learn
TEST_BIN := $(BUILD)/run_tests

.PHONY: all build test clean

all: build

build: $(BIN)

$(BIN): src/main.c
	@mkdir -p $(BUILD)
	$(CC) $(CFLAGS) -o $@ $<

test: $(TEST_BIN)
	./$(TEST_BIN)

$(TEST_BIN): tests/run_tests.c
	@mkdir -p $(BUILD)
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -rf $(BUILD)
