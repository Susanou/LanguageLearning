use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y+1;
    let y = y*2;

    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("The number of spaces is: {}", spaces);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    another_function(5);

    let x = 5;

    let y = {
        let x =3;
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("The value of the function is: {}", plus_one(5));
}

fn another_function(x: u32){

    println!("The value of x is: {}", x);

}

fn plus_one(x: i32) -> i32{
    x + 1
}
