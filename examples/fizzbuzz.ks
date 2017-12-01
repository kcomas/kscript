

TOTAL = 15
counter = 1

$ counter =< TOTAL $ {
    s = @["", ""]
    ? (counter % 3) == 0 {
        s[0] = "Fizz"
    }
    ? (counter % 5) == 0 {
        s[1] = "Buzz"
    }
    ? ??s[0]==""&?s[1]=="" == t {
        counter >> 1
    } {
        s >> 1
    }
    counter = (counter + 1)
}
