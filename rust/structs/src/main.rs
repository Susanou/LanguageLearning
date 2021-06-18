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

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    
    let rect = Rectangle{
        width: 32,
        height: 40,
    };

    println!("the area of the rectangle is {} square pixels", area(&rect));
    println!("rect = {:?}", rect);

    let square = Rectangle::square(30);
    println!("square = {:?}", square);
    println!("The are of square = {}", square.area());

    println!("rect can hold square? {}", rect.can_hold(&square));
    println!("square can hold rect? {}", square.can_hold(&rect));

}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
} 