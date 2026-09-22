# treex

A small Rust implementation of a Windows tree.exe-like directory tree viewer.

## Usage

    treex [path] [/F] [/A]

- /F displays files as well as directories.
- /A uses ASCII characters instead of Unicode line-drawing characters.
- Without a path, the current directory is used.

Examples:

    treex
    treex C:\Projects\treex /F
    treex . /F /A

## Output example

For a directory containing `src`, `tests`, `Cargo.toml`, and `README.md`:

    C:\Projects\treex
    ├── src
    │   └── main.rs
    ├── tests
    │   └── sample.txt
    ├── Cargo.toml
    └── README.md

With `/A`, the same output uses ASCII characters:

    C:\Projects\treex
    |-- src
    |   \-- main.rs
    |-- tests
    |   \-- sample.txt
    |-- Cargo.toml
    \-- README.md
