fn main() {
    let tupple = (10, 20);
    println!("Before: ({}, {})", tupple.0, tupple.1);

    let (y, x) = tupple;
    let swapped = (x, y);
    println!("After:  ({}, {})", swapped.0, swapped.1);
}
