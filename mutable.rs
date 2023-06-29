//Mutable
let mut variable_name = value;
let mut variable_name:dataType = value;
Let us understand this with an example

fn main() {
   let mut fees:i32 = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
} //Output should be:
// fees is 25000
// fees changed is 35000