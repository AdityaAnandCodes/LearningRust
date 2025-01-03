fn main() {
    let sentence = String::from("This is a sentence");
    let first_word = get_first_word(sentence);
    println!("The first word in sendence is: {}", first_word);

}

fn get_first_word(sentence : String) -> String{
    let mut ans  : String = String::from("");
    for c in sentence.chars(){
        if c == ' '{
            break;
        }
        ans.push(c);
    }
    return ans;
}