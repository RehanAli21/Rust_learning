#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    rectangle();
}

fn rectangle() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    //let width1: u32 = 30;
    //let height1: u32 = 50;

    println!(
        "The area of rectangle is {} square pixels.",
        area_using_struct(&rect1)
    );

    println!("{rect1:?}");

    println!("{rect1:#?}");
}

fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//fn area(width: u32, height: u32) -> u32 {
//width * height
//}
