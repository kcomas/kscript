
.update_refs,x,z {
    d = z
    d = 10
    "up x: " > 1
    x >> 1
    x = 5
    "up x: " > 1
    x >> 1
}

.main, {
    x = 1
    z = 2
    "x: " > 1
    x >> 1
    "z: " > 1
    z >> 1
    .update_refs,x,z
    "x: " > 1
    x >> 1
    "z: " > 1
    z >> 1
}
