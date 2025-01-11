
use std::collections::HashMap;

fn main(){
    let mut users = HashMap::new();
    users.insert(String::from("John"), 20);
    users.insert(String::from("Alice"), 25);

    let user1 = users.get("John");
    match user1 {
        Some(&age) => println!("John is {} years old", age),
        None => println!("No user found"),
    }
}