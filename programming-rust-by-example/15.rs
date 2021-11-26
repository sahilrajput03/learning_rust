// pg. 59 - Methods
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist_from_origin(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt()
    }
    fn print_dist_from_origin(&self) {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        println!("Yikes, the distance is: {}", (sum_of_squares as f64).sqrt());
    }
}

fn main() {
    let p1 = Point { x: 3, y: 4 };
    println!("distance from origin: {}", p1.dist_from_origin());
    p1.print_dist_from_origin();
}
