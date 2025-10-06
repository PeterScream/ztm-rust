// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {

    let some_var :bool = true;
    match some_var {
        true => println!("its true"),
        false => println!("its false"),
    }

    let some_int :i8 = 21;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its unknown"),
    }

}

