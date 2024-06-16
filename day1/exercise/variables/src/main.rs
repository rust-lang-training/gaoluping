fn main() {
    mut_test();

    number_test1();

    // number_test2();

    // number_test3();

    // bool_test();
}

fn mut_test() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // // let mut x = 7;
    // // println!("The value of x is: {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("{THREE_HOURS_IN_SECONDS}");
}

fn number_test1() {
    let e: u8 = b'A';
    println!("{e}");

    // let a: u32 = 1_234_567;
    // let a1 = 1_234_567u32;
    // let a2 = 1_234_567_u32;
    // println!("{a}, {a1}, {a2}");

    // let b: u8 = 0xFF;
    // let b1 = 0xFFu8;
    // let b2 = 0xFF_u8;
    // println!("{b}, {b1}, {b2}");

    // let c: i8 = 0o77;
    // let c1 = 0o77u8;
    // let c2 = 0o77_u8;
    // println!("{c}, {c1}, {c2}");

    // let d: u8 = 0b1111_0000;
    // let d1 = 0b1111_0000u8;
    // let d2 = 0b1111_0000_u8;
    // let e: u8 = b'A';
    // println!("{d}, {d1}, {d2}, {e}");
}

#[test]
fn number_test2() {
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(-1_i16 as i32, -1_i32);

    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);
}

#[test]
fn number_test3() {
    let a1 = 10u8.checked_add(100);
    let a2 = 128u8.checked_add(200);
    let a3 = 128u8.checked_mul(3);

    println!("{:?}, {:?}, {:?}", a1, a2, a3);

    let b1 = 100_u16.wrapping_mul(200);
    let b2 = 500_u16.wrapping_mul(500);
    let b3 = 900_u16.wrapping_mul(900); // 81000 % (2^16) = 23568
    println!("{:?}, {:?}, {:?}", b1, b2, b3);

    let c1 = 32760_i16.saturating_add(10);
    let c2 = (-32760_i16).saturating_sub(10);
    println!("{:?}, {:?}", c1, c2);

    let d1 = 255_u8.overflowing_sub(2);
    let d2 = 255_u8.overflowing_add(2);
    println!("{:?}, {:?}", d1, d2);
}

#[test]
fn number_test4() {
    // let num = 2;
    let res = 0.1 + 0.2;
    println!("res: {res}");
}

#[test]
fn bool_test() {
    let t = true;
    let b = 1 + (t as u8);
    println!("b = {b}");
}

#[test]
fn test_char() {
    let c1 = 'c';
    let c2 = 'π';
    let c3 = '\\';
    let c4 = '🌹';
    let c5 = '\x2A';
    let c6 = '\u{CA0}';

    println!("{c1} {c2} {c3} {c4} {c5} {c6}");
}

#[test]
fn tuple_test() {
    let tup = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred} {six_point_four} {one}");
}

#[test]
fn arr_test() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5];
    println!("{:?}", months);
    println!("{:?}", a);
    println!("{:?}", b);
}

#[test]
fn slice_test() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, 1.0, 0.707];
    let sv: &[f64] = &v;
    // let sa: &[f64] = &a;

    println!("{:?}", &v[0..2]);
    println!("{:?}", &a[0..]);
    println!("{:?}", &sv[1..3]);
}
