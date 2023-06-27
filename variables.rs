//Syntax
// let variable_name = value;          // no type specified
// let variable_name:datatype = value; //type specified
fn main5() {
    let fees = 25_000;
    let salary:f64 = 35_000.00;
    println!("fees is {} and salary is {}",fees,salary);
} //Output should be: fees is 25000 and salary is 35000.

//Immutable
fn main6() {
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

//Mutable
let mut variable_name = value;
let mut variable_name:dataType = value;
Let us understand this with an example

fn main7() {
   let mut fees:i32 = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
} //Output should be:
// fees is 25000
// fees changed is 35000