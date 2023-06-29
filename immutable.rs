//Immutable
fn main() {
    let fees = 25_000;
    println!("fees i {} ", fees);
    fees = 35_000;
    println!("fees changed is {}",fees);
} //Output should be:
// error[E0384]: re-assignment of immutable variable `fees`
//  --> main.rs:6:3
//    |
//  3 | let fees = 25_000;
//    | ---- first assignment to `fees`
// ...
//  6 | fees=35_000;
//    | ^^^^^^^^^^^ re-assignment of immutable variable

// error: aborting due to previous error(s)