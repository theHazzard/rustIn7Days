fn main() {
    println!("Hello, world!");
}

fn highest(a: i32, b: u32, c: i8) -> i32 {
    let mut res = a;

    if b as i32 > a {
        res = b as i32;
    }

    if c as i32 > a {
        res =c as i32;
    }


    res
}

#[test]
fn should_return_highest_value() {
    assert_eq!(highest(4, 3, 1), 4);
    assert_eq!(highest(4, 5, 1), 5);
    assert_eq!(highest(4, 5, 7), 7);

}
