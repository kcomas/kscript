
# Ackermann Recursive

ack = .m,n {
    m == 0 ? { n + 1; ;; }
    n == 0 ? { .m - 1,1; ;; }
    .m - 1, .m, n - 1;
}

ack.3,5; >> 1
