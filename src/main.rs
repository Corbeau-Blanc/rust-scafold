pub mod rectangle;
use rectangle::Rectangle;


fn main() {
    let rec = Rectangle{width: 10 , height: 5};
    let mut rec2 = rec.clone();

    println!("The area of rectangle a is {} " , rec2.area());
    rec2 = Rectangle::new();

    rec2.width = 0;

    if rec == rec2 {
        println!("equal");
    } else {
        println!("not equal");  
    }

    println!("Is the rec can hold rec2 ? {}", rec.can_hold(&rec2));
}
