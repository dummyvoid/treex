# treex

A small Rust implementation of a Windows tree.exe-like directory tree viewer.

## Usage

    treex [path] [/F] [/A]

- /F displays files as well as directories.
- /A uses ASCII characters instead of Unicode line-drawing characters.
- Without a path, the current directory is used.

Examples:

    treex
    treex C:\Users\dummyvoid /F
    treex . /F /A

The Windows executable is built automatically by GitHub Actions and uploaded as the treex-windows-x64 artifact.
