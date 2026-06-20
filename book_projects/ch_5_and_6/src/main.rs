#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect3 = Rectangle {
        width: 100,
        height: 30,
    };

    println!(
        "Rect 1 can hold rect 2? and answer is {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "Rect 1 can hold rect 3? and answer is {}",
        rect1.can_hold(&rect3)
    );

    //let width1: u32 = 30;
    //let height1: u32 = 50;

    println!("The area of rectangle is {} square pixels.", rect1.area());

    println!("{rect1:?}");

    println!("{rect1:#?}");
}

//fn area_using_struct(rectangle: &Rectangle) -> u32 {
//rectangle.width * rectangle.height
//}

//fn area(width: u32, height: u32) -> u32 {
//width * height
//}
