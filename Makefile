
example_dyn: some-c.o some-rust.rs
	rustc some-rust.rs -o example_dyn -C link-arg=some-c.o

example_static: libsomec.a
	rustc some-rust.rs -o example_static -l somec -L .

libsomec.a: some-c.o
	ar rcs libsomec.a some-c.o

some-c.o: some-c.c
	gcc -c some-c.c -o some-c.o

clean:
	rm example_static example_dyn some-c.o libsomec.a

