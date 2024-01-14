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

  let rect1 = Rectangle {width: 30, height: 50};
  println!(
    "The area of the rectangle is {} square pixels.",
    area_with_rectangle(&rect1)
  );
  println!("rect1 is {:?}", rect1);

  let rect2 = Rectangle {width: 30, height: 50};
  println!(
    "The area of the rectangle is {} square pixels.",
    rect2.area()
  );
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}

fn area_with_dimension(dimension: (u32,u32)) -> u32 {
  dimension.0 * dimension.1
}

fn area_with_rectangle(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}