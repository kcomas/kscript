
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
}
