fn main() {
    println!("Hello, world!");
    let n = loop_to_10();
    println!("{}", n);

    let n = for_to_10();
    println!("{}", n);

    let n = while_to_10();
    println!("{}", n);

    let n = array_loop();
    println!("{}", n);

    named_loops();
}


fn loop_to_10() -> String{
    let mut n = 0;
    let mut res = String::from("");
    loop {
        n += 1;
        res = format!("{}{}", res, n);
        if n >= 10 {break};
    } 

    res
}


fn while_to_10() -> String{
    let mut n = 0;
    let mut res = String::from("");
    while n < 10  {
        n += 1;
        res = format!("{}{}", res, n);
    } 

    res
}

fn for_to_10() -> String {
    let mut res = String::from("");
    for n in 1..11 {
        res = format!("{}{}", res, n);
    }

    res
}

fn array_loop() -> String {
    let v = vec![4,7,9];
    let mut res = String::from("");
    for n in v {
        res = format!("{}{}", res, n);
    }

    res
}

fn named_loops() {
    let v = vec![4,7,8,9,11, 10];

    'outer: for i in 0..10 {
        for n in &v {
            if i+n == 11 {
                break 'outer;
            }
            println!("{}", n);
        }
    }

}



#[test]
fn should_print_to_10() {
    let res_of_loop = loop_to_10();
    let res_of_for = for_to_10();
    let res_of_while = while_to_10();

    assert_eq!(res_of_loop, "12345678910");
    assert_eq!(res_of_for, "12345678910");
    assert_eq!(res_of_while, "12345678910");
}

#[test]
fn should_print_vector() {
    let res = array_loop();

    assert_eq!(res, "479");
}