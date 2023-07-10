fn foo(s: &mut String) {
    println!("in fn foo: {s}");
    s.push_str("");
}
fn main() {
    let mut s1 = String::from("I am a superman.");
    foo(&mut s1);
    println!("{s1}");
}
