
# Sum an array

sum = .arr,i,x {
    i == x ? { 0 ;; }
    arr[i] + .arr,i + 1,x
}

a = @[1, 2, 3, 4]
sum.a,0,4; >> 1
