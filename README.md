# PicSorter
A simple Rust command-line tool that organizes your images into folders based on their creation date.

## How to run

```bash
Usage: pic-sorter <DIRECTORY>

Arguments:
  <DIRECTORY>  

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Cross-compile for Windows to macOS

1. Add the Windows target:

```bash
rustup target add x86_64-pc-windows-gnu
```

2. Install a cross-linker (like `mingw-w64`) via Homebrew:

```bash
brew install mingw-w64
```

3. Build your project for Windows:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

4. The executable will be in:

```
target/x86_64-pc-windows-gnu/release/your_app.exe
```