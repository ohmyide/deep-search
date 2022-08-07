# deep-search（ds）
A Tiny, Static, Full-text Search with Rust

![ds](/doc/logo.png "deep-search")

## Usage

```
ds <keyword> <path>
```

## Command-line options
```
USAGE:
    ds <KETWORD> <PATH> [OPTIONS]

ARGS:
    <KETWORD>    The keywords you want to search for
    <PATH>       The file or folder you want to search

OPTIONS:
    -c               Case ignore
    -h, --help       Print help information
    -r               Cancel recursive traversal of hierarchy
    -V, --version    Print version information
```

## Demo
Add the `-r` argument to cancel traverse hierarchy recursively.

```
ds <keyword> -r
```

Add the `-c` argument to case insensitive.

```
cargo run <keyword> <path> -r -c
```


example:

```
ds helloworld
```