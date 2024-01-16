debug:
	cargo build
	g++ -o main-debug main.cpp -L ../target/debug -lrust_interop_with_cpp -lpthread -ldl

release:
	cargo build --release
	g++ -O3 -o main main.cpp -L ../target/release -lrust_interop_with_cpp -lpthread -ldl
