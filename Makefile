MINI_GMP = external/gmp/mini-gmp

.PHONY: all clean

all: factorial hello

# submodule check
$(MINI_GMP)/mini-gmp.c:
	@echo "Error: git submodules not initialized. Run:"
	@echo "  git submodule update --init"
	@exit 1

# mini-gmp example
factorial: factorial.rs libminigmp.a
	rustc factorial.rs -l minigmp -L . -o factorial

# mini-gmp example (dynamic)
factorial_dyn: factorial.rs mini-gmp.o
	rustc factorial.rs -C link-arg=mini-gmp.o -o factorial_dyn

libminigmp.a: mini-gmp.o
	ar rcs libminigmp.a mini-gmp.o

mini-gmp.o: $(MINI_GMP)/mini-gmp.c $(MINI_GMP)/mini-gmp.h
	gcc -c $(MINI_GMP)/mini-gmp.c -o mini-gmp.o

# hello world example
hello: hello.rs libhello.a
	rustc hello.rs -l hello -L . -o hello

# hello world example (dynamic)
hello_dyn: hello.rs hello.o
	rustc hello.rs -C link-arg=hello.o -o hello_dyn

libhello.a: hello.o
	ar rcs libhello.a hello.o

hello.o: hello.c
	gcc -c hello.c -o hello.o

clean:
	rm -f factorial factorial_dyn mini-gmp.o libminigmp.a hello hello_dyn hello.o libhello.a

