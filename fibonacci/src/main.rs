fn main() {
    println!("{}", fibo(4));
}


fn fibo(num:u32) -> u32{

    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1{
        return second;
    }

    for _ in 0..num-1{
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;


}