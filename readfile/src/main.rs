use std::fs::read_to_string;

fn main(){
    let result = read_to_string("hello.txt");
    match result {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error: {}", e),
    }
}

