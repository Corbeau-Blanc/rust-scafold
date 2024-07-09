pub struct Rectangle {
    pub(crate) width: u32,
    pub(crate) height: u32
}
// Implementations

impl Rectangle {
    pub fn new()-> Self {
        return Rectangle {
            width: 10,
            height: 5
        }
    }
    pub fn area(&self)-> u32 {
        return self.width * self.height;
    }
    pub fn can_hold(&self, other: &Rectangle)-> bool {
        return self.width > other.width && self.height > other.height;
    }
}

// Derive trait

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