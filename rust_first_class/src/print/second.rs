pub fn print_second() {
    let from = 'A' as u32;
    for ch in char::from_u32(from + 1).unwrap()..'z' {
        println!("{}", ch);
    }
}
