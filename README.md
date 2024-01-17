# Running Rust from C++

This is a simple example of how to run Rust code from C++. A Makefile is provided to
build the Rust code and link it into the C++ code.
[`cbindgen`](https://github.com/mozilla/cbindgen) is used to generate the C++ header
file from the Rust code and should be installed before building the code. Alternatively,
the header file can be generated manually.

Run `make` to build a debug version of the code, producing an executable called `main`
in `target/debug/`. Run `make release` to build a release version of the code, producing
an executable called `main` in `target/release/`.
