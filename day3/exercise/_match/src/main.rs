fn main() {
    println!("{}", describe_point(0, 0));
    println!("{}", describe_point(10, 0));
    println!("{}", describe_point(0, 10));
    println!("{}", describe_point(20, 10));
    println!("{}", describe_point(-20, 10));
    

    let s = "alkasd7890anjas^&#^fg";
    let mut iter = s.chars();
    loop {
        match iter.next() {
            Some(n @ '0'..='9') => println!("found a numeric char: {}", n),
            Some(c @ ('a'..='z' | 'A'..='Z')) => {
                println!("found a alphabet char: {}", c);
            },
            Some(c) => println!("found char not a numberic nor alphabet: {}", c),
            None => {
                println!("End of iteration");
                break;
            }
        }
    }

}

fn describe_point(x: i32, y:i32) -> &'static str {
    use std::cmp::Ordering::*;

    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin"!
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }

}





