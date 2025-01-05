enum Shapes {
    Rectangle ( f64 , f64),
    Circle (f64),
}


fn main() {
    let rect = Shapes::Rectangle(1.0,2.0);
    let area = print_area(rect);
    println!("Rectangle Area : {}",area);
    let circ = Shapes::Circle(2.0);
    let area = print_area(circ);
    println!("Circle area :{}",area);
}


fn print_area(shape : Shapes) -> f64 {
    match shape{
        Shapes::Rectangle(width, height) =>  width*height,
        Shapes::Circle(radius) =>  3.14*radius*radius,
    }
}