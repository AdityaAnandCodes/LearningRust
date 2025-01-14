fn main() {
    let name = String::from("Hello, world!");
    let ans = first_word(&name);
    println!("{}", ans);
}

fn first_word(str : &String) -> &str {
    let mut space_index = 0;
    for i in str.chars(){
        if i == ' '{
            break;
        }
        space_index = space_index + 1;
    }
    return &str[0..space_index]
}
