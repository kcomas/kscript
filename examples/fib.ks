
# Fibonacci Number Recursive

fib = .,n {
    n == 0 ? { 0 ;; }
    n == 1 ? { 1 ;; }
    ..,n - 1; + ..,n - 2
}

fib.,15; >> 1
