fn main() {
    // test2();


    // 宽度
    // println!("Hello {:5}!", "x");
    // println!("Hello {:1$}!", "x", 5);
    // println!("Hello {1:0$}!", 5, "x");
    // println!("Hello {:width$}!", "x", width = 5);
    // let width = 5;
    // println!("Hello {:width$}!", "x");


    // 对齐与补充格式
    // println!("Hello {:<6}!", "x");  // Hello x     !
    // println!("Hello {:>6}!", "x");  // Hello      x!
    // println!("Hello {:-<6}!", "x"); // Hello x-----!
    // println!("Hello {:^6}!", "x");  // Hello   x   !

    // 数字的符号及进制
    // println!("Hello {:+}!", 5);   // Hello +5!
    // println!("Hello {:#x}!", 27); // Hello 0x1b!
    // println!("Hello {:#X}!", 27); // Hello 0x1B!
    // println!("Hello {:#b}!", 27); // Hello 0b11011!
    // println!("Hello {:#010b}!", 27); // Hello 0b00011011!  二进制10位，不足10位用0补齐
    // println!("Hello {:05}!", 4);  // Hello 00004!
    // println!("Hello {:05}!", -4); // Hello -0004!


    // 转义
    // println!("Hello {{}}");
    // println!("{{ Hello");

    // 其它
    println!("{:?}", Some(1));
    println!("{:#?}", Some(1));

    let formt_str = format!("{:?}", Some(1));
    println!("{formt_str}");


}

#[test]
fn test1() -> i32 {
    let y = {
        // ------------ 一对儿花括号印起来的块是表达式
        let x = 3; // 字面量 3 是表达式
                   // 块中最后一个表达式如果没有尾部的分号，
                   // 那么它就是这个块表达式的返回值
        x + 1
    }; // -----------------------------------
    y // 所以，这里 y 是函数 foo() 的返回值 }
}


// fn test2() {
//     let mut count = 0;
 
//     'count_up: loop {

//         println!("count = {count}");
//         let mut remaining = 10;

//         let _ = 'inner_loop: loop {
//             println!("remaining = {remaining}");

//             if remaining == 9 {
//                 break 'inner_loop 3;
//             }
//             if count == 2 {
//                 break 'count_up;
//             }

//             remaining -= 1;
//         };

//         count+=1;
//     }

// }


#[test]
fn test3() {

    let items = [10, 20, 30, 40, 50];
    for item in items {
        println!("{item}")
    }

}


// 无法通过编译
// fn foo2() {
//     let condition = true;
//     // if condition { 5 } else { "six" };
//     let number = if condition { 5 } else { "six" };
//     println!("The value of number is: {number}");
// }

