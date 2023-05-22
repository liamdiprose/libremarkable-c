target/debug/liblibremarkable_c.so: src/lib.rs cbindgen.toml build.rs
	cargo build

target/debug/liblibremarkable_c.a: src/lib.rs cbindgen.toml build.rs
	cargo build

test/bin/hello_world: test/hello_world.c target/debug/liblibremarkable_c.so
	mkdir -p test/bin
	gcc -o $@ -Ltarget/debug -llibremarkable_c test/hello_world.c

test/bin/hello_world_static: test/hello_world.c target/debug/liblibremarkable_c.a
	gcc -c -Wall -fpic $< -o hello_world.o
	gcc -o $@ -static hello_world.o -Ltarget/debug -llibremarkable_c


test_shared: test/bin/hello_world
	LD_LIBRARY_PATH=target/debug $<

test: test/bin/hello_world_static
	$<

.PHONY = clean
clean:
	rm -rf test/bin
