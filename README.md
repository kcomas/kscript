
# Kscript

Work in progress see [Overview](design/OVERVIEW.md)

### Current Status
* [Trello](https://trello.com/b/IevrZUGw/kscript)

### Usage
```
    kscript <options> file.ks
```

#### Options
```
--help --log-stdout --log-stderr --read-stdin
```

* Debug
```
    cargo run -- file.ks
```

* Release
```
    cargo build --release
    ./target/release/kscript file.ks
```
