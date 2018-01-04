
.update_index,x {
    x[1] = 1
}

.main, {
    x = @[1, "+", 2, "+", 3, "+", 4 * 2,
        @[" ", "A", "S", "D", "F"]
    ]
    "Array: " > 1
    x >> 1
    "7th index: " > 1
    x[
        @["err1", 4 +2 * 2 - 1, "err2"][x[0]]
    ] >> 1
    "7th after update: " > 1
    .update_index,x[7]
    x[7][2] = 2
    x[7] >> 1

    p = @[]
    "Empty: " > 1; p >> 1
    "Push Array: " > 1
    p @< @[1, 2]
    p >> 1
    "Push to sub array: " > 1
    p[0] @< 3
    p[0] @< 4
    p >> 1
    "Pop sub array: ": > 1
    # vars must be declared before poped to
    p[0] @> 1 # throw array pop
    p[0] @> p[0][0]
    p >> 1
    "To Var d: " > 1
    d = 1
    p[0] @> d
    d >> 1
}
