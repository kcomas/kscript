
# Recursive Example Of Fib

fib = { |n, &g|
    ? n =< 1 {
        r = 1
    } {
        r = (g|(n - 1), g| + g|(n - 2), g|)
    }
    r
}

result = fib|30, fib|

@["Recursive Result: ", result] >> 1
