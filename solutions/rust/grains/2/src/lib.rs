use std::u64;

pub fn square(s: u32) -> u64 {
    return 2u64.pow(s - 1);
}

pub fn total() -> u64 {
    let total: u128 = 2u128.pow(64) - 1;
    total.try_into().unwrap()
}
