//Invoking a Function
fn main12(){
    //calling a function
    fn_hello();
}
//Defining a Function
fn fn_hello(){
    println!("hello from function fn_hello ");
}//Output should be:
//hello from function fn_hello

//Returning Value from a Function
//with return statement
fn function_name() -> return_type {
    //statements
    return value;
 }
//shorthand syntax without return statement
fn function_name() -> return_type {
    value //no semicolon means this value is returned
 }
 fn main13(){
    println!("pi value is {}",get_pi());
 }
 fn get_pi()->f64 {
    22.0/7.0
 }//Output should be:
 //pi value is 3.142857142857143

 //Function with Parameters
 //Pass by Value
 fn main14(){
    let no:i32 = 5;
   mutate_no_to_zero(no);
   println!("The value of no is:{}",no);
}

fn mutate_no_to_zero(mut param_no: i32) {
   param_no = param_no*0;
   println!("param_no value is :{}",param_no);
 }//Output should be:
//  param_no value is :0
// The value of no is:5

//Pass by Reference
fn main15(){
    let mut no:i32 = 5;
    mutate_no_to_zero(&mut no);
    println!("The value of no is:{}",no);
 }
 fn mutate_no_to_zero(param_no:&mut i32){
    *param_no = 0; //de reference
 }//Output should be:
 //The value of no is 0.

 //Passing string to a function
 fn main16(){
    let name:String = String::from("TutorialsPoint");
    display(name); 
    //cannot access name after display
 }
 fn display(param_name:String){
    println!("param_name value is :{}",param_name);
 }//Output should be:
 //param_name value is :TutorialsPoint