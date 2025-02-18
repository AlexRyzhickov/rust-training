fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

}

// fn main() {
//     let mut s = String::from("hello");
//
//     {
//         let r1 = &mut s;
//         println!("{}", r1);
//     } // r1 goes out of scope here, so we can make a new reference with no problems.
//
//     let r2 = &mut s;
//     println!("{}", r2);
// }
