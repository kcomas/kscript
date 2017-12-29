
# Ackermann Recursive

.ack,m,n {
    m == 0 ? { n + 1 ;; }
    n == 0 ? { .ack,m - 1, 1; ;; }
    .ack,m - 1,.ack,m,n - 1;
}

.main, {
    .ack,3,4; >> 1
}
