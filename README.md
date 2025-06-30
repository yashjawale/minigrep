# Minigrep

Minigrep is a simple command-line utility written in Rust that mimics the functionality of the Unix `grep` command. It searches for lines in a file that match a specified query string. The application supports both case-sensitive and case-insensitive search.

## Features

- **Case-Sensitive Search**: Search for lines containing the exact query string.
- **Case-Insensitive Search**: Search for lines containing the query string, ignoring letter casing.

## Usage

```sh
cargo run <query> <file_path>
```

- `<query>`: The string you want to search for.
- `<file_path>`: The path to the file in which to search.

### Example

```sh
cargo run to poem.txt
```

This command will search for the string "to" in the file `poem.txt`.

## Environment Variables

- `IGNORE_CASE`: If set, the search will be case-insensitive.

## Structure

- **src/main.rs**: Contains the main function which sets up the command-line interface and runs the program.
- **src/lib.rs**: Contains the core functionality, including search functions and the `Config` struct.

## Testing

Unit tests are provided in the `src/lib.rs` file. To run the tests, use:

```sh
cargo test
```

> [!NOTE]
> This program was created by following [Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) of [The Rust Book](https://doc.rust-lang.org/book/title-page.html)

---

<a href="https://yashjawale.github.io/" target="_blank"><img style="height: 22px;" src="https://raw.githubusercontent.com/yashjawale/.github/main/docs/logo.svg" alt="Yash Jawale"/></a>
