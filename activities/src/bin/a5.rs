// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {

    let mut var :i8 = 1;
    loop {

        var = var +1;

        println!("current var {:?}",var);
        if var == 4 {
         break;
        }
    }
    

}
