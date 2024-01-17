debug:
	cargo build
	cbindgen -o src/rust.h
	g++ -o target/debug/main src/main.cpp -L target/debug -lrust_interop_with_cpp -lpthread -ldl

release:
	cargo build --release
	cbindgen -o src/rust.h
	g++ -O3 -o target/release/main src/main.cpp -L target/release -lrust_interop_with_cpp -lpthread -ldl
