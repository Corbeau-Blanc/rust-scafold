struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn new()-> Self {
        return Rectangle {
            width: 10,
            height: 5
        }
    }
    fn area(&self)-> u32 {
        return self.width * self.height;
    }
    fn can_hold(&self, other: &Rectangle)-> bool {
        return self.width > other.width && self.height > other.height;
    }
}

impl Clone for Rectangle {
    fn clone(&self) -> Self {
        return Rectangle { width: self.width, height: self.height };
    }

    fn clone_from(&mut self, source: &Rectangle) {
        self.width = source.width;
        self.height = source.height;
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        return self.width == other.width && self.height == other.height;
    }
}
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
