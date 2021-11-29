// ? pg. 79, refer e.g., 27.rs for the earlier reference for operator overloading.
#[allow(unused)]
macro_rules! op {
    (+ $_self:ident : $self_type:ty, $other:ident $expr:expr) => {
        // ?  ^ this line is absolute same in other case except the - operator.
        impl ::std::ops::Add for $self_type {
            type Output = $self_type;

            fn add($_self, $other: $self_type) -> $self_type {
                $expr
            }
        }
    };

    (- $_self:ident : $self_type:ty, $other:ident $expr:expr) => {
            impl ::std::ops::Sub for $self_type {
                type Output = $self_type;

                fn sub($_self, $other: $self_type) -> $self_type {
                    $expr
                }
            }
    };
}

#[derive(Debug)]
struct Point {
    x: i8,
    y: i8,
}
op!(+ self:Point, other {
    Point {
        x: self.x + other.x,
        y: self.y + other.y,
    }
});
op!(- self:Point, other {
    Point {
        x: self.x - other.x,
        y: self.y - other.y,
    }
});

fn main() {
    println!("hello");
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("p3: {:?}", p3)
}
