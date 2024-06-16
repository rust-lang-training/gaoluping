use std::io;

fn main() {
    println!("请输入一个正整数");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Fail to read line");
    let n: u32 = n.trim().parse().expect("请输入正整数");

    if is_prime(n) {
        println!("{}是质数", n);
    } else {
        println!("{}不是质数", n); // 2, 3, 5, 7, 11, ..., 44623, ...
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        } else {
        }
        i += 1;
    }

    true
}
