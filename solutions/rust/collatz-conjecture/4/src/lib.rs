pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut n = n;

    let mut steps: u64 = 0;

    loop {
        if n == 1 {
            return Some(steps);
        }

        if n.is_multiple_of(2) {
            n /= 2
        } else {
            n = n * 3 + 1
        }

        steps += 1;
    }
}
