const PI : f32 = 3.14159;
use std::io;

fn main(){
    println!("Enter the radius of the circle : ");
    loop{
        let mut radius = String::new();
        io::stdin().read_line(&mut radius).expect("Failed to read line");
        match radius.trim().parse::<f32>(){
            Ok(num)=>{
                let area = PI * num * num;
                println!("The area of the circle is : {}", area);
                break;
            }
            Err(_)=>{
                println!("Please input a number!");
                continue;
            }
        }
    }
}