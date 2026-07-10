pub fn reverse(input: &str) -> String {
    let mut chars: Vec<char> = vec![];
    for c in input.chars() {
        chars.push(c);
    }

    let mut reversed: String = String::new();
    for i in chars.iter().rev() {
        reversed.push(*i);
    }
    reversed
}
