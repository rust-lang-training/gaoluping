fn main() {
    println!("Hello, world!");



}


#[test]
fn str_test1() {
    let s6 = r###"
    This raw string starts width 'r###"'
    Therefore it does not end until we reach a quote mark ('"')
    followed immediately by three pound signs ('###'):
"###;
    println!("{s6}");
}

#[test]
fn str_test2() {
    let str1 = "𝑒";
    let str1len1 = str1.len();
    let str1len2 = str1.chars().count();
    println!("{str1}: {str1len1}, {str1len2}");

    let str2 = "a";
    let str2len1 = str2.len();
    let str2len2 = str2.chars().count();
    println!("{str2}: {str2len1}, {str2len2}");
}

#[test]
fn slice_test2() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, 1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    println!("sv = {:?}", sv);
    println!("sa = {:?}", sa);
    println!("sa = {:?}", &sa[1..]);
    println!("sa = {:?}", &sv[1..]);
    println!("sa = {:?}", &sv[..3]);
}

