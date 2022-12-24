fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 50,
    };
    println!(
        "The area of the rectangle1 is {} square pixels. Rect2 is {} sp. Can fit = {}",
        rect1.area(), rect2.area(), rect1.can_hold(&rect2)
    );
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        if rect2.area() <= self.area() {
            return true;
        }
        false
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}