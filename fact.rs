pub fn fact(n:usize) -> usize {
    if n < 2 {
        return 1;
    }
    return n * fact(n - 1);
}
