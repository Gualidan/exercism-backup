pub fn collatz(n: u64) -> Option<u64> {
    (n > 0).then(|| {
        let mut n = n;
        let mut steps: u64 = 0;
        while n != 1 {
            if n.is_multiple_of(2) {
                n /= 2;
            } else {
                n = n * 3 + 1;
            }
            steps += 1;
        }
        steps
    })
}
