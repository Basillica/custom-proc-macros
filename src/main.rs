extern crate lib1;
extern crate lib2;
extern crate custom_macros;

use custom_macros::{QueryBuilder3, GettersSetters, Builder, Factory};

#[derive(QueryBuilder3, Debug)]
#[use_attrs_with_query]
#[table_name(sample2)]
struct User2 {
    id: Option<i32>,
    name: Option<String>,
    email: Option<String>,
    age: Option<i32>,
}

pub struct User {
    username: String,
    first_name: String,
    last_name: String,
}

impl core::fmt::Debug for User {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> ::core::fmt::Result {
        f.debug_struct(
            "User"
        )
        .field("username", &self.username)
        .field("first_name", &self.first_name)
        .field("last_name", &self.last_name)
        .finish()
    }
}


#[derive(GettersSetters)]
pub struct User3 {
    id: u32,
    username: String,
}

#[derive(Builder)]
pub struct User4 {
    id: u32,
    username: String,
    email: String,
}

fn main() {
    // Initialize the struct using the `new` function
    let mut user = User3::new(1, String::from("Alice"));

    // Use the getter methods
    println!("ID: {}", user.id());
    println!("Username: {}", user.username());

    // Use the setter methods
    user.set_id(2);
    user.set_username(String::from("Bob"));

    println!("Updated ID: {}", user.id());
    println!("Updated Username: {}", user.username());

    ////////////////77
    // Create a User using the builder pattern
    let user = User4::builder()
        .id(1)
        .username("Alice".into())
        .email("alice@example.com".into())
        .build()
        .unwrap();

    println!("User: {}, {}, {}", user.id, user.username, user.email);

    ////////////////////////////
    #[derive(Factory)]
    pub struct Product {
        product_id: u32,
        #[factory]
        name: String,
        price: f64,
        #[factory]
        age: i32,
    }
    let product = Product::new_product(1001, 34.56);
    println!("Product ID: {}, Name: {}, Price: ${}, age: {}", product.product_id, product.name, product.price, product.age);


    /////////////////////////
    let user = User2 {
        id: Some(1),
        name: None,
        email: Some("user@example.com".to_string()),
        age: None,
    };

    println!("the frigging user: {:?}", user);
    
    let query = user.build_query();
    println!("the frigging query {}", query);
    println!("Main function called");
    lib1::shitty();
    lib2::stuffy();
}