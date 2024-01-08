fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let guess = "42".parse::<u32>().expect("Not a number!");

    println!("The value of guess is: {}", guess);

    let tapple = (500, 6.4, "2");

    let (_x, y, _z) = tapple;
    println!("The value of y is: {}", y);

    let a = [1,2,3,4,5];
    println!("Please enter an array index.");
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("Failed to read line");
    let _index: usize = index.trim().parse().expect("Index entered was not a number");

    another_function();
}

fn another_function() {
    println!("Another function.");
}