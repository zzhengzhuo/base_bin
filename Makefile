CXX=riscv64-unknown-elf-g++
CC=riscv64-unknown-elf-gcc

build/base_bin: main.c base_lib/target/riscv64imac-unknown-none-elf/release/libbase_lib.a 
	$(CXX) -Ibase_lib -o $@ $^ 

build/cc_base_bin: main.c base_lib/target/riscv64imac-unknown-none-elf/release/libbase_lib.a 
	$(CC) -Ibase_lib -o $@ $^ 

rust:
	cd base_lib && cargo build --release --target riscv64imac-unknown-none-elf -v 

clean:
	rm build/*
	cd base_lib && cargo clean