pub fn egg_count(display_value: u32) -> usize {
    let mut eggs = 0;
    let mut value = display_value;
    while value != 0 {
        if value & 1 == 1 {
            eggs +=1
        }

        value >>= 1;
    }
    eggs
}
