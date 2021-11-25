// ? pg 56
// ? Clone types
// ? An alternative to using references is to clone values. By cloning a value, we don't move it. To be able to clone a point, we can add derive to it:

fn print_point(point: Point){
    println!("x:{}, y:{}", point.x, point.y)
}

#[allow(unused_variables)]
fn main(){
    let p1 = Point {x: 1, y: 2};
    let p2 = p1.clone();
    let p3 = p1.clone();
    let p4 = p1.clone();
    // p1 is never moved but copied to p2, p3, p4 vars
    print_point(p1.clone());

    println!("p1 = {:?}", p1.x);//snip: pdb
}

// Here Debug is optional in below statement.
#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}