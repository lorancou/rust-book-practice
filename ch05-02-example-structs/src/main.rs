fn main() {
    // An Example Program Using Structs

    let width1 = 30;
    let height1 = 50;

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Refactoring with Tuples

    let rect1 = (30, 50);

    fn area_tuple(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // Refactoring with Structs: Adding More Meaning

    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    fn area_struct(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    // Adding Useful Functionality with Derived Traits

    //println!("rect1 is {}", rect1); error[E0277]: `main::Rectangle` doesn't implement `std::fmt::Display`
    //println!("rect1 is {:?}", rect1); error[E0277]: `main::Rectangle` doesn't implement `std::fmt::Display`

    #[derive(Debug)] // Annotation
    struct DebugRectangle {
        width: u32,
        height: u32,
    }

    let rect1 = DebugRectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); // pretty-print
}
