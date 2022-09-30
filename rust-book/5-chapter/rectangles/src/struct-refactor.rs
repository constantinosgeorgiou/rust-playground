// Refactoring with structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // width: 30,
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// &Rectangle is an immutable borrow of a struct Rectangle.
// Borrows the struct rather than take ownership from main.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
