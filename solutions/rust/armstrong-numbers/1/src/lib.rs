pub fn is_armstrong_number(num: u32) -> bool {
        let s = num.to_string();
    let number_of_digits = s.len() as u32;
    let sum: u32 = s
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(number_of_digits))
        .sum();
    sum == num
}
