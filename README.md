# treex

A small Rust implementation of a Windows tree.exe-like directory tree viewer.

## Usage

    treex [path] [/F] [/A]

- /F displays files as well as directories.
- /A uses ASCII characters instead of Unicode line-drawing characters.
- /ABOUT displays program information.
- Without a path, the current directory is used.

Examples:

    treex
    treex C:\Users\dummyvoid /F
    treex . /F /A
    treex /ABOUT

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

### About

    $ treex /ABOUT
    treex 0.1.0
    A small Rust implementation of a Windows tree.exe-like directory tree viewer.

    Written in Rust.
    License: MIT
    Source: https://github.com/dummyvoid/treex

The Windows executable is built automatically by GitHub Actions and uploaded as the `treex-windows-x64` artifact.
