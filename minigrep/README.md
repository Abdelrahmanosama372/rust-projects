# minigrep-rust

`minigrep-rust` is a simple command-line utility written in Rust, inspired by the Unix `grep` tool. It allows users to search for a given query within a specified file and print matching lines. This project serves as a practical exercise in Rust programming, showcasing basic file I/O, command-line argument parsing, error handling, and unit testing.

## Features

- **Search Functionality**: Search for a given query within a specified file.
- **Case Sensitivity**: Supports both case-sensitive and case-insensitive search.
- **Unit Testing**: Includes unit tests for verifying the functionality of search algorithms.
- **Error Handling**: Provides robust error handling for file I/O operations and command-line argument parsing.

## Usage
- To use `minigrep-rust`, clone this repository and compile the code using Rust's package manager, Cargo:

```sh
git clone https://github.com/your_username/minigrep-rust.git
cd minigrep-rust
cargo build --release
```
- Once compiled, you can run minigrep-rust from the command line as follows:
```sh
cargo run -- <ignore_case_flag> <query> <file_path> 
```
- Replace <span style="color: yellow;">ignore_case_flag</span> with an optional flag (`i` or `nothing`) for case-insensitive search, <span style="color: yellow;">query</span> with the term you want to search for, <span style="color: yellow;">file_path</span> with the path to the file you want to search in
- example
```sh
cargo run -- i hello sample.txt
```