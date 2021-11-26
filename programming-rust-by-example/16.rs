// pg. 59 - Methods
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

fn main() {
    let mut p1 = Point { x: 3, y: 4 };
    p1.translate(2, 2);
    println!("point: {:?}", p1);
}
