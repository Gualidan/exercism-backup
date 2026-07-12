pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut one_time_check = true;
    let mut steps: u64 = 0;

    loop {
        if one_time_check {
            if n == 1 {
                return Some(steps);
            }
            if n == 0 {
                return None;
            }
            one_time_check = false;
        }
        if n.is_multiple_of(2) {
            n /= 2
        } else {
            n = n * 3 + 1
        }
        steps += 1;
        if n == 1 {
            return Some(steps);
        }
    }
}
