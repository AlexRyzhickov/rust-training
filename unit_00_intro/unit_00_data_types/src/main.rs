//https://doc.rust-lang.org/book/ch03-02-data-types.html

const PI: f32 = 3.1415926535897932384626433832795;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let a: i32 = 0; // immutable
    let b = 1; // immutable
    println!("a:{}, b:{}", a, b);
    // a = a + 1; // cannot assign twice to immutable variable
    let mut c: i32 = 0; // mutable
    c = c + 1;
    println!("c:{}", c);
    println!("pi :{}, secs in 3 hour: {}", PI, THREE_HOURS_IN_SECONDS);

    c = i32::MAX;
    // c = c + 1; attempt to add with overflow
    println!("c:{}", c);

    // Scalar Types
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Number literals	Example
    // Decimal          98_222
    // Hex              0xff
    // Octal            0o77
    // Binary           0b1111_0000
    // Byte (u8 only)	b'A'

    // Floating-Point Types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x:{}, y:{}", x, y);

    // The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t:{}, f:{}", t, f);

    // The Character Type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c:{}, z:{}, c:{}", c, z, heart_eyed_cat);

    // Compound Types
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup_0:{}, tup_1: {}, tup_2: {}", tup.0, tup.1, tup.2);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x:{}, y:{}, z:{}", x, y, z);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);

    // String types
    let s = "as";
    println!("s:{}", s);

    // The Array Type
    let arr = [1, 2, 3, 4, 5];
    println!("arr:{:?}", arr);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr:{:?}", a);
    let a = [3; 5];
    println!("arr:{:?}", a);
    let first = a[0];
    let second = a[1];
    println!("first:{}, second:{}", first, second);
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("months: {:?}", months);
}
