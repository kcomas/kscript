
.add,a,b,c,d,e {
    d_copy = d
    d_copy >> 1
    d_copy ** (b + a) * c + e
}

.main, {
    a = 1
    b = 2
    a + b >> 1
    test = "\"a\"sdf#\n"
    test > 1
    .add,1,2,3,5,1; >> 1
    9 / (2 + 1) * 3.3 >> 1
    boolean = t
    boolean == t ? { "true is true" >> 1 }
    "Hello" + " " + "World" >> 1
    (2 * (1 + 1)) >> 1
    ("Grouped" >> 1)
}
