
.add,a,b,c,d,e {
    f = d
    f >> 1
    # a + b * c / f >> 1
    f ** (b + a) * c + e
}

.main, {
    .add,1,2,3,5,1; >> 1
    9 / (2 + 1) * 3.3 >> 1
}
