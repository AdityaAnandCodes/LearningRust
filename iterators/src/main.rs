fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter{
        match val {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("something else"),
            }
        }
    }


