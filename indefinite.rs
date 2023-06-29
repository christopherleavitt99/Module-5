//Indefinite Loop
fn main() {
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