func findComplement(num int) int {
    n := 1;
    for n <= num {
        n *= 2;
    }
    return n - 1 - num;
}
