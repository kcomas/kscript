
.update_x,x {
    x = 12
    "up 2 x: " > 1
    x >> 1
    3
}

.update_refs,x,z {
    d = z
    d = 10
    "up x: " > 1
    x >> 1
    x = 5
    "up x: " > 1
    x >> 1
    2 + .update_x,x
}

# Program entry

.main, {
    x = 1
    z = 2
    "x: " > 1
    x >> 1
    "z: " > 1
    z >> 1
    ret = .update_refs,x,z;
    "Return value: " > 1
    ret >> 1
    "x: " > 1
    x >> 1
    "z: " > 1
    z >> 1
}
