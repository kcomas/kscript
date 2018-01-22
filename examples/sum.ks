
# Sum an array

sum = .arr,i {
    i == @? arr ? { 0 ;; }
    arr[i] + .arr,i + 1
}

a = @[1, 2, 3, 4]
sum.a,0; >> 1
