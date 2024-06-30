
fn main() {
    println!("输入2个正整数，用空格分隔");

    let mut nums_input = String::new();
    std::io::stdin().read_line(&mut nums_input).expect("Fail to read line");
    let nums_input = nums_input.trim();
    let mut nums_parse = [0u32; 2];
    let mut index = 0;

    for num in nums_input.split(" ") {
        let num: u32 = num.parse().expect("请输入正整数");
        nums_parse[index] = num;
        index += 1;
        if index == 2 {
            break;
        }
    }

    let ans = gcd(nums_parse[0], nums_parse[1]);
    println!("{}和{}的最大公约数为{}", nums_parse[0], nums_parse[1], ans);
}


fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a < b {
        return gcd(b, a);
    }
    while b > 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}