pub mod second;

pub fn print_first() {
    let from = 'Z' as u32;
    for ch in (char::from_u32(from + 1).unwrap()..'a').rev() {
        println!("{}", ch);
    }
}
