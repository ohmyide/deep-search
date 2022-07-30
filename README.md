# deep-search
A Tiny, Static, Full-text Search with Rust

## Usage

```
cargo run <keyword> <path>
```

## Command-line options
```
USAGE:
    deep-search <KETWORD> <PATH> [OPTIONS]

ARGS:
    <KETWORD>    The keywords you want to search for
    <PATH>       The file or folder you want to search

OPTIONS:
    -c               Case ignore
    -h, --help       Print help information
    -r               Traverse hierarchy recursively
    -V, --version    Print version information
```

## Demo
Add the `-r` argument to traverse hierarchy recursively.

```
cargo run <keyword> <path> -r
```

Add the `-c` argument to case insensitive.

```
cargo run <keyword> <path> -r -c
```


demo:

```
cargo run Rust ./tests ./tests -r -c
```