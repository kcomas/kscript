# Kscript

## Overview

A scripting language written in Rust.

### Build with

```
cargo build --release
```

### Run with

```
./target/release/kscript file.ks
```

### Debug with
```
KSCRIPT_DEBUG=1 ./target/release/kscript file.ks
```

#### Examples can be found in the examples folder

## Documentation

### Types
* Bools
    * true: t
    * false: f
* Integers
* Floats
* Strings

### Comments
Comments start with a # and go to the end of the line

```
# This is a comment
.main, {
    # This is a comment
}
```

### Functions
All programs must have a main function

Functions are labeled by a period followed by a name and args separated by commas

All args are passed by reference see the mem.ks example for what this means

There must be a comma after the function name
```
.add,x,y {
    x + y
}

.main, {
    # add 1 + 2 and write to standard out with a newline
    .add,1,2; >> 1
}
```

Function calls must end with a newline or have a ; after the call

### Assignment =
Variables are assigned with = by deep copy
```
.main, {
    a = 1
    b = 2
    # Prints 3 to stdout
    a + b >> 1
}
```

### Io
Can all write operations are done with arrows

Writes can be done to files and file descriptors

Fds:

* 1: STDOUT
* 2: STDERR

#### Output

##### Write >
Write to stdout
```
.main, {
    "Hello World\n" > 1
}
```

##### Append >>
Append to stdout
```
.main, {
    "Hello World" >> 1
}
```
