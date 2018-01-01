
.update_refs,x {
    "up x: " > 1
    x >> 1
    x = 5
    "up x: " > 1
    x >> 1
}

.main, {
    x = 1
    "x: " > 1
    x >> 1
    .update_refs,x
    "x: " > 1
    x >> 1
}
