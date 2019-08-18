#[derive(Debug)]
pub struct Bed {
    size: i32,
    count: u32,
}

#[derive(Debug)]
pub enum Room {
    Kitchen(i32),
    Bedroom(Bed),
    Lounge
}

fn main() {
    let t = Room::Kitchen(4);
    let br = Room::Bedroom(Bed {size: 50, count: 2});
    println!("Hello from the {:?}!", t);

    match t {
        Room::Kitchen(n) => println!("Kitchen with {} rooms", n),
        d => println!("{:?}", d)
    }

    if let Room::Bedroom(n) = br {
        println!("{:?}", n);
    }
}
