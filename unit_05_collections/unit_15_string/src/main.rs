#![allow(unused)]
fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    for c in "Зд".chars() {
        println!("{c}");
    }
}
