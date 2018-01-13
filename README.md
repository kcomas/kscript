# Kscript

A scripting language written in [Rust](https://www.rust-lang.org)

## Usage
[Install rust](https://www.rust-lang.org)

### Building Release
```
cargo build --release
```

### Running
```
./target/release/kscript <flags> file.ks
```

Use -h or --help for a full list of flags

### Examples

Example scripts are provided in the examples folder

```
./target/release/kscript ./examples/fib.ks
```

### Debugging

Use -d or --debug flag to print debugging info to STDOUT

Use -df <file\> or --debug-file <file\> to print the debugging info to a file
