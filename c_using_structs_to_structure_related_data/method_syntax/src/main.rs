#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of a rectangle with a height of {}px and a width of {}px is {}px.",
        rect1.height,
        rect1.width,
        rect1.area()
    );
}
