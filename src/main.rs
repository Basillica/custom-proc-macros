extern crate lib1;
extern crate custom_macros;

use lib1::lib1_add;
use custom_macros::{
    SqlQueryDerive, DeriveGetterSetter,
    BuiderPattern, FactoryPattern,
};

// for the sql query macro
#[derive(Debug, SqlQueryDerive)]
#[table_name(anthony)] // attribute macro
struct Student {
    first_name: Option<String>,
    last_name: Option<String>,
    id: Option<i64>,
}

// for the setter-getter macro
#[derive(Debug, DeriveGetterSetter)]
struct Person{
    first_name: String,
    last_name: String,
    address: String,
    age: i64,
}

// for the builder patter macro
#[derive(Debug, BuiderPattern)]
struct User {
    first_name: String,
    last_name: String,
    address: String,
    age: i64,
}

// for the factory pattern
#[derive(Debug, FactoryPattern)]
struct Product {
    #[factory]
    id: u32,
    name: String,
    price: i64,
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        {
            $a+$b
        }
    };
}

fn main() {
    println!("Hello, world!");
    println!("the result of 3 + 5 = {}", add!(3, 5));
    println!("the result from lib1: {}", lib1_add(3, 5));

    // sql builder example
    let student = Student{
        id: None,
        first_name: Some(String::from("tonie")),
        last_name: None,
    };

    let query = student.build_query();
    println!("our query: {query}");

    // setter-getter example
    let person = Person{
        first_name: "tonie".to_string(),
        last_name: "etienne".to_string(),
        address: "germany".to_string(),
        age: 34,
    };
    println!("first_name: {}", person.get_first_name());

    // builder patter example
    let user00 = User{
        first_name: "tonie".to_string(),
        last_name: "etienne".to_string(),
        address: "germany".to_string(),
        age: 34,
    };

    let user11 = User::builder()
        .first_name("tonie".into())
        .last_name("etienne".into())
        .address("germany".into())
        .age(34)
        .build()
        .unwrap();

    assert_eq!(user00.first_name, user11.first_name);
    println!("user00: {:?}", user00);
    println!("user11: {:?}", user11);

    // factory pattern example
    let product = Product::new_product(String::from("value"), 23);
    println!("id : {}, name: {} and price: {}", product.id, product.name, product.price);
}
