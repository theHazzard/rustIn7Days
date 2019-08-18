#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    height: i32,
    shoesize: i32,
}

impl User {
    fn simple_string(&self) -> String {
        format!("{} - {} - {}cm - shoe: {}", self.name, self.age, self.height, self.shoesize)
    }

    fn grow(&mut self, h: i32) {
        self.height += h; 
    }

    fn die(self) {
        println!("Dead {}", self.simple_string());
    }
}

fn main() {
    let mut u = User {
        name: "Enzo".to_string(),
        age: 31,
        height: 178,
        shoesize: 45
    };

    println!("User is {}", u.simple_string());
    u.grow(20);
    println!("User is {}", u.simple_string());

    u.die();
}
