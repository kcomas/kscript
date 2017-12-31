
.add,a,b,c,d {
    f = d
    f >> 1
    # a + b * c / f >> 1
    f ** b + a * c
}

.main, {
    .add,1,2,3,5; >> 1
}
