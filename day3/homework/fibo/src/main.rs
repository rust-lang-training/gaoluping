use std::io;

fn main() {
    println!("输入斐波那契数列的第几项n");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Fail to read line");
    let n: u32 = n.trim().parse().expect("请输入正整数");

    println!("n = {n}");

    let result = fibo(n);

    println!("计算结果为{result}");
}

// 递归
fn fibo(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        n => fibo(n - 1).wrapping_add(fibo(n - 2))
    }
}
