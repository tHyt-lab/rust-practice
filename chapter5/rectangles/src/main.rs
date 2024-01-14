#[derive(Debug)]
// TypeScriptのinterfaceのようなもの
struct Rectangle {
    width: u32,
    height: u32,
}

// 構造体にメソッドを定義する
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_dimension(rect)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_rectangle(&rect1)
    );
    println!("rect1 is {:?}", rect1);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

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
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_dimension(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_with_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
