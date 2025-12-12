# romcom: the romantic compiler

[![status: pre-alpha](https://img.shields.io/badge/status-pre--alpha-red)](./README.md)
[![rust: 2021](https://img.shields.io/badge/rust-2021-black?logo=rust)](./Cargo.toml)
[![docker: ready](https://img.shields.io/badge/docker-ready-2496ED?logo=docker&logoColor=white)](./Dockerfile)

`romcom` is a tiny, toy compiler that takes a very small expression language and turns it into x86-64 assembly, then assembles/links it into a native executable that prints the computed result.

This is **pre-alpha** research code: expect breaking changes and rough edges.

## What it supports (so far)

- Integers (e.g. `420`, `-7`)
- Postfix increment/decrement (`x++`, `x--`)
- `let` bindings (e.g. `let x = 420; x`)

## Quick start (local)

### Prerequisites

- Rust (edition 2021)
- NASM (`nasm`)
- Clang (`clang`) (used as the linker driver)

### Build and run

The compiler expects a file path and reads a single line of source.

1. Compile a program (example input: `420.int`):
   - `cargo run --release -- 420.int`
2. Run the compiled executable:
   - Windows: `.\build\out.exe`
   - Linux/macOS: `./build/out.exe`

The output should be the computed integer result followed by a newline.

## Docker

This repo includes a Docker setup that installs the required toolchain (Rust, NASM, Clang) and runs the compiler in a container.

1. Build the image:
   - `docker build -t romcom:latest .`
2. Run the compiler against a program file and write build outputs to your host `build/` folder:
   - PowerShell:
     - `docker run --rm -v "${PWD}\420.int:/program.int:ro" -v "${PWD}\build:/app/build" romcom:latest /program.int`
   - Bash:
     - `docker run --rm -v "$PWD/420.int:/program.int:ro" -v "$PWD/build:/app/build" romcom:latest /program.int`

## Repo layout

- `src/` — compiler, parser, AST, assembly emission
- `build/` — generated artifacts (`out.asm`, `out.obj`, `out.exe`)

## Roadmap (aspirational)

- More expressions and operators
- Better errors and diagnostics
- Cleaner backend + more tests

## License & contributions

- This project is released under the [BSD 2-Clause License](LICENSE).
- romcom is an experimental, pre-alpha research toy. It is not accepting contributions right now and issues are not prioritized; feel free to fork and iterate independently.
