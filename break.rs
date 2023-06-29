fn main(){
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