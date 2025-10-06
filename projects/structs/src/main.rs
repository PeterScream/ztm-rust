struct GroceryItem {
    stock: i32,
    price: f64,
}

struct User {
    username: String,   
    email: String,
    sign_in_count: u32,
    active: bool,   

}

fn main() {
    println!("-- struct -- ");

    let chicken = GroceryItem {
        stock: 10,
        price: 2.99,
    };
    println!("stock: {:?}", chicken.stock);
    println!("price: {:?}", chicken.price);

    // -- Lets Get Rusty 'Structs in Rust'
    // https://www.youtube.com/watch?v=n3bPhdiJm9I&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=6

    let mut user1: User = User {        //  you have to make all struct mutable you cant make just 1 field mutable. (2)
        email: String::from("email-addr@piltover.com"),
        username: String::from("Jinx"),
        active: true,
        sign_in_count: 1,
    };

    let name: String = user1.username;
    user1.username = String::from("Vi"); // changing the name of the username in the existing struct (1)

}

// we can also use funcitons to construct new instances of user 
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    };

// (3) -- now we can use our funciton to create a new user

  let user2 = build_user(
    email: String::from("Vander@thedude.com"),
    username: String::from("Vander"));

// (4) -- another convinient feature is that we can create new instances of a struct using existing instances
    let user3: User {
        email: String::from("silco@sickMan.com")
        username: String::from("Silco")
        .. user2 // for the rest of the fields just give us whatever the user2 has
    }    

}

/* because our farguments have the same name as our fields in this function we can do: (simplify - remove double fields)
    this is called field init shot syntax

fn build_user(email: String, username: String) -> User {
    User {
        email: 
        username: 
        active: true,
        sign_in_count: 1,
    };
}  

*/