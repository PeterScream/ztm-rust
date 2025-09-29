// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid milileters
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


enum EnumFlavor {
    Sparkling, 
    Sweet,
    Fruity,
} 

struct StrDrink {
  flavor: EnumFlavor,
  fluid_mil: f64,
}

fn print_drink(drink: StrDrink){
    match drink.flavor {
        EnumFlavor::Sparkling => print!("flavor: Sparkling"),
        EnumFlavor::Sweet => print!("flavor: Sweet"),
        EnumFlavor::Fruity => print!("flavor: Fruity"),
    }
    println!("oz: {:?}", drink.fluid_mil);

}

fn main() {

    let sparkling_drink = StrDrink {
        flavor: EnumFlavor::Sparkling,
        fluid_mil: 6.0
    };
    print_drink(sparkling_drink);
   
    let sweet_drink = StrDrink {
        flavor: EnumFlavor::Sweet,
        fluid_mil: 6.0
    };
    print_drink(sweet_drink);

    let fruity_drink = StrDrink {
        flavor: EnumFlavor::Fruity,
        fluid_mil: 6.0
    };
    print_drink(fruity_drink);

    


}
