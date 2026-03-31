# shell-rust

A POSIX-compliant shell built in Rust. Supports command pipelines, I/O redirection, interactive line editing, tab completion, and persistent history — written as part of the [CodeCrafters "Build Your Own Shell"](https://app.codecrafters.io/courses/shell/overview) challenge.

## Features

- **Interactive line editing** — cursor movement, Home/End, Backspace, Delete
- **History navigation** — Up/Down arrows, persisted across sessions via `HISTFILE`
- **Tab completion** — autocompletes builtin and PATH-discovered executables
- **Pipelines** — pipe builtins and external commands together with `|`
- **I/O redirection** — `>`, `>>`, `2>`, `2>>`, `1>`, `1>>`
- **Quoting and escaping** — single quotes, double quotes, backslash escapes
- **Tilde expansion** — `~` resolves to `$HOME`

## Builtins

| Command   | Description                                      |
|-----------|--------------------------------------------------|
| `cd`      | Change directory; `cd` alone goes to `$HOME`     |
| `echo`    | Print arguments to stdout                        |
| `pwd`     | Print current working directory                  |
| `exit`    | Exit with optional exit code (default 0)         |
| `type`    | Show whether a command is a builtin or external  |
| `history` | Display history; supports `-r`/`-w`/`-a` flags   |

## Project Structure

```
src/
├── main.rs          # REPL entry point
├── lib.rs           # Pipeline execution engine
├── shell.rs         # Terminal UI and keyboard input (termion)
├── lexer.rs         # Tokeniser — handles quotes, escapes, operators
├── parser.rs        # Converts tokens to a pipeline of commands
├── context.rs       # Shell state: history and current buffer
├── builtins.rs      # Builtin command factory
├── builtins/        # Individual builtin implementations
├── external.rs      # External command lookup and execution
├── redirection.rs   # Redirect enum and operator parsing
├── writer.rs        # File write/append utilities
└── error.rs         # Error types
```

## Running

```sh
cargo run
```

Or use the provided wrapper:

```sh
./your_program.sh
```

## Dependencies

- [`termion`](https://crates.io/crates/termion) — raw terminal mode, key input, cursor control
- [`anyhow`](https://crates.io/crates/anyhow) — flexible error handling
- [`thiserror`](https://crates.io/crates/thiserror) — derive macros for error types
- [`bytes`](https://crates.io/crates/bytes) — buffer management for piped output
