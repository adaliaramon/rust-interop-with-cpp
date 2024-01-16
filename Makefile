debug:
	cargo build
	g++ -o target/debug/main src/main.cpp -L target/debug -lrust_interop_with_cpp -lpthread -ldl

release:
	cargo build --release
	g++ -O3 -o target/release/main src/main.cpp -L target/release -lrust_interop_with_cpp -lpthread -ldl
