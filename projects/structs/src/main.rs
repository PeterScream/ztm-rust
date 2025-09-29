struct GroceryItem {
    stock: i32,
    price: f64,
}

fn main() {
    println!("-- struct -- ");

    let chicken = GroceryItem {
        stock: 10,
        price: 2.99,
    };
    println!("stock: {:?}", chicken.stock);
    println!("price: {:?}", chicken.price);
}
