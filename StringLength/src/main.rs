fn main() {
    let name  = String::from("Harikirat");
    let len = get_str_len(name);
    println!("Length of name is: {}", len);
}


fn get_str_len(str : String) -> usize {
    str.chars().count()
}