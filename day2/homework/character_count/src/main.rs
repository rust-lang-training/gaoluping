use std::io;

fn main() {
    println!("请输入一个字符串");

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Fail to read line");
    let s = s.trim();

    let mut results = [0u32; 26];

    for c in s.to_lowercase().chars() {
        let index = c as u8;
        if index >=97 && index < (97 + 26) {
            results[(index - 97) as usize] += 1;
        }
    }

    for i in 0..26 {
        println!("char {} count is {}", ((i as u8) + 97) as char, results[i]);
    }
}
