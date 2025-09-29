
fn coordinates() -> (i32, i32){
    (1,5)
}


fn main() {
    println!(" -- tuples -- ");

    let (x,y) = coordinate();

    if y >5 {
        println!(">5");
    } else if y < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}
