//For Loop
fn main8() {
    for x in 1..11{ //11 is not inclusive
        if x==5{
            continue;
        }
        println!("x is {}",x);
    }
}//Output should be:
// x is 1
// x is 2
// x is 3
// x is 4
// x is 6
// x is 7
// x is 8
// x is 9
// x is 10

//Indefinite Loop
fn main9() {
    let mut x = 0;
    while x < 10{
        x+=1;
        println!("inside loop x value is {}",x);
    }
    println!("outside loop x value is {}",x);
}//Output should be:
// inside loop x value is 1
// inside loop x value is 2
// inside loop x value is 3
// inside loop x value is 4
// inside loop x value is 5
// inside loop x value is 6
// inside loop x value is 7
// inside loop x value is 8
// inside loop x value is 9
// inside loop x value is 10
// outside loop x value is 10

fn main10(){
    //while true

   let mut x = 0;
   loop {
      x+=1;
      println!("x={}",x);

      if x==15 {
         break;
      }
   }
}//The break statement is used to take the control out of a construct. 
//Using break in a loop causes the program to exit the loop.
//Output should be:
//x=1
// x=2
// x=3
// x=4
// x=5
// x=6
// x=7
// x=8
// x=9
// x=10
// x=11
// x=12
// x=13
// x=14
// x=15

//Continue Statement
fn main11(){
    let mut count = 0;

    for num in 0..21 {
        if num % 2==0 {
            continue;
         }
         count+=1;
      }
      println! (" The count of odd values between 0 and 20 is: {} ",count);
      //outputs 10
   }