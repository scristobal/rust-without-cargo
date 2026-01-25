EXTERNAL = external
GMP = $(EXTERNAL)/gmp
MINI_GMP = $(GMP)/mini-gmp
BUILD = build

.PHONY: all clean

all: $(BUILD)/factorial $(BUILD)/hello $(BUILD)/dist $(BUILD)/errno $(BUILD)/state

$(BUILD):
	mkdir -p $(BUILD)

# submodule check
$(MINI_GMP)/mini-gmp.c:
	@echo "Error: git submodules not initialized. Run:"
	@echo "  git submodule update --init"
	@exit 1

# mini-gmp example
$(BUILD)/factorial: factorial.rs $(BUILD)/libminigmp.a | $(BUILD)
	rustc --edition=2024 factorial.rs -l minigmp -L $(BUILD) -o $(BUILD)/factorial

# mini-gmp example (dynamic)
$(BUILD)/factorial_dyn: factorial.rs $(BUILD)/mini-gmp.o | $(BUILD)
	rustc --edition=2024 factorial.rs -C link-arg=$(BUILD)/mini-gmp.o -o $(BUILD)/factorial_dyn

$(BUILD)/libminigmp.a: $(BUILD)/mini-gmp.o
	ar rcs $(BUILD)/libminigmp.a $(BUILD)/mini-gmp.o

$(BUILD)/mini-gmp.o: $(MINI_GMP)/mini-gmp.c $(MINI_GMP)/mini-gmp.h | $(BUILD)
	gcc -c $(MINI_GMP)/mini-gmp.c -o $(BUILD)/mini-gmp.o

# hello world example (Rust calls C calls Rust)
$(BUILD)/hello: hello.rs $(BUILD)/hello.o | $(BUILD)
	rustc --edition=2024 hello.rs -C link-arg=$(BUILD)/hello.o -o $(BUILD)/hello

$(BUILD)/hello.o: hello.c | $(BUILD)
	gcc -c hello.c -o $(BUILD)/hello.o

# dist example (Rust callback to C's qsort)
$(BUILD)/dist: dist.rs | $(BUILD)
	rustc --edition=2024 dist.rs -o $(BUILD)/dist

# errno example (error handling across FFI)
$(BUILD)/errno: errno.rs | $(BUILD)
	rustc --edition=2024 errno.rs -o $(BUILD)/errno

# state example (atomic + thread-local across FFI)
$(BUILD)/state: state.rs $(BUILD)/state.o | $(BUILD)
	rustc --edition=2024 state.rs -C link-arg=$(BUILD)/state.o -o $(BUILD)/state

$(BUILD)/state.o: state.c | $(BUILD)
	gcc -c state.c -o $(BUILD)/state.o

clean:
	rm -rf $(BUILD)

