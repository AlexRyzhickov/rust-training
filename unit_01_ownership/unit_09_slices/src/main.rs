fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let s = "Hello, world!";
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    println!("{:?}, {:?}", slice1, slice2);
}
