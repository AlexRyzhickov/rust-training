fn main() {
    let s1 = String::from("hello");
    let h = s1[0];
}

// $ cargo run
// Compiling collections v0.1.0 (file:///projects/collections)
// error[E0277]: the type `str` cannot be indexed by `{integer}`
// --> src/main.rs:3:16
// |
// 3 |     let h = s1[0];
// |                ^ string indices are ranges of `usize`
// |
// = help: the trait `SliceIndex<str>` is not implemented for `{integer}`, which is required by `String: Index<_>`
// = note: you can use `.chars().nth()` or `.bytes().nth()`
// for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
// = help: the trait `SliceIndex<[_]>` is implemented for `usize`
// = help: for that trait implementation, expected `[_]`, found `str`
// = note: required for `String` to implement `Index<{integer}>`
//
// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `collections` (bin "collections") due to 1 previous error

