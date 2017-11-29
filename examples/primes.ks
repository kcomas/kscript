
# Print The First 100 Primes Line By Line

pos = 1
prime = 2
total = 100

printer = { |prime, &pos|
    @[pos, ": ", prime] >> 1
    pos = (pos + 1)
}

isprime = { |prime|
    c = 2
    r = t
    $ c < prime $ {
        ? (prime % c) == 0 {
            r = f
            c = prime
        }
        c = (c + 1)
    }
    r
}

$ pos =< total $ {
    ? t == isprime|prime| {
        printer|prime, pos|
    }
    prime = (prime + 1)
}
