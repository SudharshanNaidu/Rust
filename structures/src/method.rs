/*
    This example demonstrates the usage of methods in structures
*/

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(dim: u32) -> Self { //Associate function
        Self {
            height: dim,
            width: dim,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };
    
    let rect2 = Rectangle {
        height: 10,
        width: 40,
    };
    
    let rect3 = Rectangle {
        height: 60,
        width: 30,
    };

    dbg!("Area of {rect1.height} * {rect1.width} rectangle is {rect1.area}");
    dbg!("Area of {rect2.height} * {rect2.width} rectangle is {rect2.area}");
    dbg!("Area of {rect3.height} * {rect3.width} rectangle is {rect3.area}");

    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3: {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(5);
    println!("Area of squaure with {} * {} is {}", sq.width, sq.height, sq.area());


}