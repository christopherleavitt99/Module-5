//For Loop
fn main() {
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