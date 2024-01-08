fn main() {
  let x = 5;

  if x < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  let condition = false;
  let number = if condition { 5 } else { 6 };
  println!("The value of number is: {}", number);

  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {}", result);

  // fizzbuzz
  for number in 1..=100 {
    if number % 15 == 0 {
      println!("fizzbuzz");
    } else if number % 3 == 0 {
      println!("fizz");
    } else if number % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", number);
    }
  }

  println!("fibonachi: {}", fibonachi(45));
}

fn fibonachi(n: u32) -> u32 {
  if n == 0 {
    return 0;
  } else if n == 1 {
    return 1;
  }

  fibonachi(n - 1) + fibonachi(n - 2)
}