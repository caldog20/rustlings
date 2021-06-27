// move_semantics5.rs
// Make me compile without adding, removing, or changing any of the
// lines in `main()`.
// Execute `rustlings hint move_semantics5` for hints :)



fn main() {
    let mut x = 100;
    let y = &mut x; // Ref to X = 100
    *y += 100;
    let z = &mut *y; // Ref to y->X = 100
    *z += 1000;
    assert_eq!(x, 1200);
}
