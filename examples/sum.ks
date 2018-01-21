
# Sum an array

sum = .arr,i,x,s {
    i == x ? { s ;; }
    s = s + arr[i]
    i = i + 1
    .arr,i,x,s;
}

a = @[1, 2, 3, 4]
sum.a,0,4,0; >> 1
