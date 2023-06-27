//Declare a Variable
fn main1() {
    let company_string = "TutorialsPoint";  //string type
    let rating_float = 4.5;                 //float type
    let is_growing_boolean = true;          //boolean type
    let icon_char = "💗";                   //unicode character type
    println!("company name is: {}",company_string);
    println!("rating is: {}",rating_float);
    println!("company is growing :{}",is_growing_boolean);
    println!("icon is: {}",icon_char);
} //Output should be:
// company name is: TutorialsPoint
// company rating on 5 is:4.5
// company is growing: true
// company icon is: ♥

//Scalar Types
fn main2() {
    let result = 10;    // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10
    let count:usize = 30;
    println!("result value is: {}",result);
    println!("sum is {} and age is: {}",sum,age);
    println!("mark is: {} and count is {}",mark,count);
} //Output should be:
// result value is 10
// sum is -10 and age is 20
// mark is 10 and count is 30

//Integer Overflow
fn main3() {
    let age:u8 = 255;

    // 0 to 255 only allowed for u8
    let weight:u8 = 256;    //overflow value is 0
    let height:u8 = 257;    //overflow value is 1
    let score:u8 = 258;     //overflow value is 2
}//Output should be:
// age is 255
// weight is 0
// height is 1
// score is 2

//Float
fn main4() {
    let result = 10.00;         //f64 by default
    let interest:f32 = 8.35;
    let cost:f64 = 15000.600    //double precision

    println!("result value is {}",result);
    println!("interest is {}",interest);
    println!("cost is {}",cost);
}//Output should be:
// interest is 8.35
//cost is 15000.6
