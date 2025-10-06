// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal



fn sub(a: i32, b: i32) -> i32 {
    a-b
}



fn first_name() {
    println!("My name is: Piko");
}

fn last_name() {
    println!("My surename is: Scream");
}

fn main() {
    first_name();
    last_name();

    let sum = 2-2;
    let value = 10 -5;
    let divison = 10 /2;
    let mult = 5*5;

    let new_sum = sub(3,3); 

}
