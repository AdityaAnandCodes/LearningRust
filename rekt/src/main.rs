struct Rect {
    width : u32,
    height : u32,
}


impl Rect {
    fn area(&self){
        println!("Area of rectangle is : {} sq units", self.width * self.height);
    }
    fn perimeter(&self){
        println!("Perimeter of rectangle is : {} units", 2 * (self.width + self.height));
    }
}

fn main(){
    let rekt : Rect = Rect{
        width : 30,
        height : 50,
    };
    rekt.area();
    rekt.perimeter();
}