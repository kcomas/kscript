
# Iterative Example Of Fib

total = 50
a = 1
b = 0

$total => 0$ {
   tmp = a
   a = (a + b)
   b = tmp
   total = (total - 1)
}

@["Iter Result: ", b] >> 1
