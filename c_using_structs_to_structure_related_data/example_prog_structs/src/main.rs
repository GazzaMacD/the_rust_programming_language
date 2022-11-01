#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    show_area();
}

fn show_area() {
    let width1 = 30;
    let height1 = 50;

    println! {
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    }
    println!("\n----------\n");

    println! {
        "The area of the rectangle is {} square pixels.",
        tuple_area((width1, height1))
    }

    println!("\n----------\n");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Using struct here is much more explicit and clear, hence better programming.\n");

    println!("The rect was {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rect1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
