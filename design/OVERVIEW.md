
# KScript

An **apl-ish**, **write-only** scripting language to be used like bash

## Modes
* The application can run in two modes
* Standard - run with out logger and only stop on error
* Debug - log all operations and stop on error

## Variables
* Not Hoisted
* Can be constants that cannot be changed after assignment unless deleted
* Passed by copy

### Variable Types
* Null
    * Cannot be assigned, only variables that have been deleted have this type
* Boolean
    * Assigned as ```t``` or ```f```
* Integer
    * Signed handled by rust
* Float
    * Signed handled by rust
* String
    * Strings are characters between "", non object variables can be interpolated with $
* Files
    * Files are read as characters between ''
* Object
* Function

### Objects
* Arrays and Dictionaries
* Passed by deep copy

#### Array
* Arrays are always mixed typed of and can be initialized with other variables
    * Eg if we have a var of integer named a and create and array with ```@[a]``` the resulting array has a copy of the var so it is now [number]
* Declared with ```@[...]```
* Items separated by comma
* Accessed with ```array[integer|(variable with integer)|(constant with integer]]``` throws error otherwise

#### Dictionaries
* Mixed type with key value pairs
* Keys are strings
* Values can be any type
    * Eg if we have a var with an array and create a dictionary with ```%["1": a]``` the resulting dictionary is now ```%["1": copy of array in a]```
* Declared with ```%[key: value]```
* Pairs separated by comma
* Accessed with ```dictionary[string|(variable with string)|(constant with string)]```

### Functions
* Does not have access to the parent scope
* Defined by ```{ |arguments| actions }```
* Arguments are separated by comma
* Can take in variables via reference or copy
    * Eg ```|a, &b|``` a is passed by copy and b by reference
* Are called by ```variable|args|```

### Variable Naming
* Variable can only be named with alpha characters of upper or lower case and numbers
* Upper case variables are constants and exported as environment variables
* Lower case variables are fixed typed reassign-able variables

## Comments
* Comments are denoted by ```#``` and goes to the end of the line

## End Statement
* Statements are ended with ```;``` or a new line

## Scalar Operations
* Does not apply to functions and objects

### Assignment =
* Can assign a names variable to data

### IO
* Can write to Files, 1 (STDOUT), 2 STDERR
* Can read from File, 0 (STDIN)

#### > Write
* Eg ```"hello" > 1``` write hello to STDOUT

#### >> Append
* Eg ```"test" >> 'file_name'``` append test to a file name

#### \< Read
* Eg ```a < 'test.txt'``` Read the contents of test.txt into var a
* Eg ```b < 0``` Read STDIN until return into var b

#### \<\< Read Append
* Eg ```a << 'text.text``` append the contents of test.txt to var a
* Eg ```b << 0``` Read STDIN until return into var b

### Math ()
* Can be used with integer and float variables
* Operations between an integer and float makes a float
* All operations in parenthesis are evaluated with order of operations
    * Eg ```((1 + 3) + 2 * 4 / 2)```

#### Addition +
#### Subtraction -
#### Multiplication *
#### Division /
#### Modulus %
#### Exponent ^
#### Parenthesis ()

## Conditionals
* Checks value and value
* Only can be done in if and loop statements

### Equals ==

### Not Equals !=

### Greater then >
* Only applies to numbers

### Greater then or equal to >=
* Only applies to numbers

### Less then \<
* Only applies to numbers

### Less then or equal to \<=
* Only applies to numbers

### And &

### Or ^

## Single Commands
* Takes single argument

### Run Commands ```! @["commands", "...args"]```
* Take an array of the command and the args and run
* Returns an array of [exit_code, STDOUT, STDERR]

## If statement
* ```?comparision{true statments}{false statements}```
* ```?comparision```
    * This just returns a boolean

## Loops
* ```$condition${while condition}```

## System Commands
* All commands start with ```\```

### Exit \\\\code
* Exits the program with code
