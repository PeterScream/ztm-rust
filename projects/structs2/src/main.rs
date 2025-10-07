// Lets get rusty - Structs in Rust
// https://www.youtube.com/watch?v=n3bPhdiJm9I&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=6


/*  (1)
    fn area (width: u32, height: u32) -> u32 {
        width * height
    }
*/

/*  (2)
    fn area (dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1 
    }
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self)-> u32 {
        self.width * self.height
    }
}

/*  (3)
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
*/

fn main() {
   
    /* (1)
        let width1 : u32 = 30;
        let height1: u32 = 50;
    */

    // let rect: (u32, u32) = (30,50);   // (2)

    let rect: Rectangle =  Rectangle {  // this is the owner of the Rectangle
        width: 30,                     // the struct definition (lines 17-20) is just a blueprint
        height: 50
    }; 

    /* (3)
        println!(
            "\n The area of the rectangle is {} square pixels.", area(&rect)
        );
    */


    println!(
        "\n The area of the rectangle is {} square pixels.", rect.area()
    );
    
    println!("\n rect: {:#?}", rect);  // '\n' (new line before the string)

} 