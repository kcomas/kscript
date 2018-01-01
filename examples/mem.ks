
.update_x,x {
    x = 12
    "up 2: " > 1
    x >> 1
}

.update_refs,x,z {
    d = z
    d = 10
    "up x: " > 1
    x >> 1
    x = 5
    "up x: " > 1
    x >> 1
    .update_x,x
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
