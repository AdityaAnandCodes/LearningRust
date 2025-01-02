use std::io;

fn main() {
    println!("Enter a number : ");
    loop{
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");

    match num.trim().parse::<u32>(){
        Ok(num) => {
            if num % 2 == 0 {
                println!("{} is an even number", num);
                break;
            } else {
                println!("{} is an odd number", num);
                break;
            }
        },
        Err(_)=>{
            println!("Please input a number!");
            continue;
        }
    }
    }
}
