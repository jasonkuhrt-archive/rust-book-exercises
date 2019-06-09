#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        other_rect.height <= self.height && other_rect.width <= self.width
    }
}

// multiple impl blocks possible
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let fat_rect = Rectangle {
        width: 30,
        height: 50,
    };
    let mini_rect = Rectangle::square(10);
    println!(
        "The area of the rectangle {:?} is {} square pixels.\nCan it hold this other rectangle {:?}? {}",
        fat_rect,
        fat_rect.area(), //alt: area(&some_rect)
        mini_rect,
        fat_rect.can_hold(&mini_rect)
    );
}

// alternative area calculator
// fn area(rect: &Rectangle) -> u32 {
//     rect.height * rect.width
// }
