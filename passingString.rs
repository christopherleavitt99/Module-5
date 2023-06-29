//Passing string to a function
fn main(){
    let name:String = String::from("TutorialsPoint");
    display(name); 
    //cannot access name after display
 }
 fn display(param_name:String){
    println!("param_name value is :{}",param_name);
 }//Output should be:
 //param_name value is :TutorialsPoint