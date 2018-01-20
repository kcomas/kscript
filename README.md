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

If not file is provided a repl is opened

Use -h or --help for a full list of flags

### Examples

Example scripts are provided in the examples folder

```
./target/release/kscript ./examples/fib.ks
```

### Debugging

Use -d or --debug flag to print debugging info to STDOUT

Use -df <file\> or --debug-file <file\> to print the debugging info to a file

## Documentation

### Types

#### Atoms passed by value
* Bool represented as t and f
* Integer
* Float
* Char

#### Collections passed by reference
* String
* Function
* Array

### Assignment =
Copies by value or reference depending on the type

### Comparison

* == Checks if two items are the same, throws error if types are not the same

### Numeric Operations
All number ops check if the two types are integers, if not the operation is done as if they where floats

* \+ Addition
* \- Subtraction
* \* Multiplication
* \\ Division
* \\\\ Modulo
* \*\* Exponential

### Grouping Operations
All commands can be grouped with ()

```
    1 + 2 * 3 >> 1 # 7
    (1 + 2) * 3 >> 1 # 9
```

### String operations

* \+ concat two strings to make a new string
```
    new_stirng = "a" + "b"
```

* \+\+ modifies a string in place by adding a new string on to the end of it
```
    my_string ++ "a"
```

* \* repeat a string a number of times into a new string
```
    new_string = "a" * 5
```

### Array/String Operations

* \[\] get an element in the array/string
```
    a = @[1, 2, 3]
    a[1] >> 1
```

* =\[\] get and update an element in the array string
```
    a = @[1, 2, 3]
    a =[1] 5
    a >> 1
```

### Functions
Functions are data that can be passed

Have the signature
```
.x,y,z... { <body> }
```
They can take any number of arguments and implicitly return the last value
```
    add = .x,y { x + y }
    add.1,2; >> 1
```

If nothing is on the stack when called it will try to call itself

### Return
To leave a function use ;;
```
    e = .{ 1 == 1 ? { 2 ;; } 4 }
    e.; >> 1
```
This will also return the last value on the stack if any

### IO
1. STDOUT
2. STDERR

* \> IoWrite
    * STDOUT, STDERR writes without a new line
```
    "Test" > 1
```

* \>\> IoAppend
    * STDOUT, STDERR writes with a new line
```
    "Test >> 1
```
