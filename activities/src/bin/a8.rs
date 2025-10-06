enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    drink_flavor: Flavor, // Descriptive field from original Option 2
    fluid_ml: f64,
}

fn print_drink(beverage: Drink) {
    // Parameter renamed for clarity
    match beverage.drink_flavor {
        // Use the new parameter name
        Flavor::Sparkling => print!("flavor: sparkling"),
        Flavor::Sweet => print!("flavor: sweet"),
        Flavor::Fruity => print!("flavor: fruity"),
    }
    println!("ml: {:.1}", beverage.fluid_ml);
}

fn main() {
    let sparkling_drink = Drink {
        drink_flavor: Flavor::Sparkling,
        fluid_ml: 6.0,
    };
    print_drink(sparkling_drink); // Caller unchanged

    let sweet_drink = Drink {
        drink_flavor: Flavor::Sweet,
        fluid_ml: 10.0,
    };
    print_drink(sweet_drink);

    let fruity_drink = Drink {
        drink_flavor: Flavor::Fruity,
        fluid_ml: 12.0,
    };
    print_drink(fruity_drink);
}