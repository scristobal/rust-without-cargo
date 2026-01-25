EXTERNAL = external
GMP = $(EXTERNAL)/gmp
MINI_GMP = $(GMP)/mini-gmp
BUILD = build

.PHONY: all clean

all: $(BUILD)/factorial $(BUILD)/hello

$(BUILD):
	mkdir -p $(BUILD)

# submodule check
$(MINI_GMP)/mini-gmp.c:
	@echo "Error: git submodules not initialized. Run:"
	@echo "  git submodule update --init"
	@exit 1

# mini-gmp example
$(BUILD)/factorial: factorial.rs $(BUILD)/libminigmp.a | $(BUILD)
	rustc factorial.rs -l minigmp -L $(BUILD) -o $(BUILD)/factorial

# mini-gmp example (dynamic)
$(BUILD)/factorial_dyn: factorial.rs $(BUILD)/mini-gmp.o | $(BUILD)
	rustc factorial.rs -C link-arg=$(BUILD)/mini-gmp.o -o $(BUILD)/factorial_dyn

$(BUILD)/libminigmp.a: $(BUILD)/mini-gmp.o
	ar rcs $(BUILD)/libminigmp.a $(BUILD)/mini-gmp.o

$(BUILD)/mini-gmp.o: $(MINI_GMP)/mini-gmp.c $(MINI_GMP)/mini-gmp.h | $(BUILD)
	gcc -c $(MINI_GMP)/mini-gmp.c -o $(BUILD)/mini-gmp.o

# hello world example
$(BUILD)/hello: hello.rs $(BUILD)/libhello.a | $(BUILD)
	rustc hello.rs -l hello -L $(BUILD) -o $(BUILD)/hello

# hello world example (dynamic)
$(BUILD)/hello_dyn: hello.rs $(BUILD)/hello.o | $(BUILD)
	rustc hello.rs -C link-arg=$(BUILD)/hello.o -o $(BUILD)/hello_dyn

$(BUILD)/libhello.a: $(BUILD)/hello.o
	ar rcs $(BUILD)/libhello.a $(BUILD)/hello.o

$(BUILD)/hello.o: hello.c | $(BUILD)
	gcc -c hello.c -o $(BUILD)/hello.o

clean:
	rm -rf $(BUILD)

