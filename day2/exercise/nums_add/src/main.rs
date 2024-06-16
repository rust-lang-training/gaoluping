use std::io;

fn main() {
    println!("输入5个非负整数，用空格分隔");

    let mut nums_input = String::new();
    io::stdin().read_line(&mut nums_input).expect("Fail to read line");
    let nums_input = nums_input.trim();
    let mut nums_parse = [0u32; 5];
    let mut index = 0;

    for num in nums_input.split(" ") {
        let num: u32 = num.parse().expect("请输入非负整数");
        nums_parse[index] = num;
        index += 1;
        if index == 5 {
            break;
        }
    }

    let total = arr_add(&nums_parse[..]);

    println!("数字总和为{}", total);

}

fn arr_add(arr: &[u32]) -> u32 {
    let mut res = 0;
    for num in arr {
        res += num;
    }
    res
}
