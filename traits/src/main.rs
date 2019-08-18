pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type =Output:Point;
    fn add(self, other: Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let a = Point {
        x: 5,
        y: 10,
    };
    let b = Point {
        x: 10,
        y: 15,
    };

    let res = a + b;
    println!("{:?}", res);
}

