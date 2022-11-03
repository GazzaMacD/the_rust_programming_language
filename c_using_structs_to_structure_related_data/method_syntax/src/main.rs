#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// ** note that &self is shorthand for self: &Self where Self is the struct the method belongs to.
//Just as in the las program we borrow an immutable reference to the Rectangle instance as we are
// just reading the data.
// It is possible to mutate the rectangle with &mut self, a mutable borrow.
// It is also possible to take ownwership of self with just self, but this is rare.
// So to summarize, Rust knows from the method self signature whether it is doing one of the following:
//a. reading self --> (&self,....)
//b. mutating self --> (&mut self,....)
//c. consuming self --> (self,....)

// ********* Getters ***********
// we can also define a getter function which simply returns the value of some data on
// the struct. This way we can keep the data private.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of a rectangle with a height of {}px and a width of {}px is {}px.",
        rect1.height,
        rect1.width,
        rect1.area()
    );
    // Using the getters instead, which will apply when using modules
    println!("\n --- using getters --- \n");
    println!(
        "The area of a rectangle with a height of {}px and a width of {}px is {}px.",
        rect1.height(),
        rect1.width(),
        rect1.area()
    );
    // Using the getters instead, which will apply when using modules
    println!("\n --- using other parameters than self --- \n");
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
