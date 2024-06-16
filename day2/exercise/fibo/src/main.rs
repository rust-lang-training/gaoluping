use std::io;

fn main() {
    println!("输入斐波那契数列的第几项n");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Fail to read line");
    let n: u32 = n.trim().parse().expect("请输入正整数");

    println!("n = {n}");

    // let result = fibo1(n);
    let result = fibo2(n);

    println!("计算结果为{result}");
}

// 递归
// fn fibo1(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     };
//     if n == 1 {
//         return 1;
//     };
//     fibo1(n - 1).wrapping_add(fibo1(n - 2))
// }

// 循环
fn fibo2(n: u32) -> u32 {
    if n == 0 {
        return 0;
    };
    if n == 1 {
        return 1;
    };
    let mut pre: u32 = 0;
    let mut cur: u32 = 1;

    for _ in 2..=n {
        let next = cur.wrapping_add(pre);
        pre = cur;
        cur = next;
    }
    cur
}
