
enum Direction {
    Left,
    Right,
}

fn main() {
    println!("      ---- enum ----");

    let go = Direction::Right;
    match go {
       Direction::Left => println!("    ... go left"), 
       Direction::Right => println!("   ... go right"),
    }

}
