fn main() {
  let _str = "Hello, world!";
  let _str2 = String::from("Hello, world!");

  let x = 5;
  let y = x;
  let z = x;

  println!("x = {}, y = {}, z = {}", x, y, z);

  let s1 = String::from("Hello");
  let len = calculate_length(&s1);
  println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}