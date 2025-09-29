// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum ColourList {
    Black,
    White,
    Red,
    Purpule,
}

fn main() {
    let colour_select = ColourList::Black;
    match colour_select {
        ColourList::Black => println!(" colour is: black"),
        ColourList::White => println!(" colour is: white"),
        ColourList::Red => println!(" colour is: red"),
        ColourList::Purpule => println!(" colour is: purpule"),
    }
}
