fn main() {
    // println!("Hello, world!");
    // foo();
    // let x = 5;
    // let y = x;

    // let s1  = String::from("hello");
    // let s2 = s1;
    // // println!("{s1}");
    // println!("{s2}");

    // let s  = String::from("hello");
    // let s2 = s.clone();
    // takes_onwershipt(s);

    // let x = 5;
    // makes_copy(x);

    // println!("{s}");

    // let names = [
    //     String::from("Hohn"),
    //     String::from("Tom"),
    //     String::from("Penny"),
    //     String::from("Sheldon"),
    // ];

    // for i in 0..4 {
    //     let mut s = names[i];
    //     println!("{}", s);
    // }

    // println!("names[0] = {}", names[0]);

    {
        // let s = String::from("Hello world");
        // let bytes = s.into_bytes();
        // println!("{}", s);
    }

    {
        let x: i32 = 10;
        let y = &x;
        // assert!(y == 10);
        assert!(y == &10);
        assert!(*y == 10);
    }

    {
        let mut x: i32 = 10;
        let z = &mut x;
        *z = 20;
        // println!("x = {x}"); // error
        println!("z = {z}");
        println!("x = {x}");
    }

    {
        let x: i32 = 10;
        let y: &i32;
        y = &x;
    }

    {
        let s1 = String::from("hello");
        let s2 = &s1;
        let s3 = &s2;
        let len = calculate_length(&&s1);
        let len2 = calculate_length(&s3);
        println!("The length of '{}' is {} {}", s1, len, len2);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }

    {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        // let r3 = &mut s;
        println!("r1={}", r1);
        println!("r2={}", r2);
        // println!("r3={}", r3);
    }

    // {
    //     // 以下代码无法通过编译
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     println!("r = {}", r);
    // }

    // {
    //     // 此代码无法通过编译
    //     fn longest(x: &str, y: &str) -> &str {
    //         if x.len() > y.len() {
    //             x
    //         } else {
    //             y
    //         }
    //     }
    // }

    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }

    {
        // // 以下代码无法通过编译
        // let mut s = String::from("hello world");
        // let rs = &s;

        // s.push_str(" I'm rust");

        // println!("The string is: {}", rs);
    }

    {
        // // 以下代码无法通过编译
        // let s1 = String::from("hello world");
        // let rs1 = &s1;
        // let s2 = s1;
        // println!("string is {}", rs1);
    }

    {
        // 以下代码无法通过编译
        // let bytes = gen_string().as_bytes();
        // println!("bytes are: {:?}", bytes);

        // 建议：只要是有值的地方，就一定给它一个所有者，可以避免很多问题
        let s = gen_string();
        let bytes = s.as_bytes();
        println!("bytes are: {:?}", bytes);

        fn gen_string() -> String {
            String::from("Hello world")
        }
    }

}

fn takes_onwershipt(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

// fn foo() {
//     {
//         // 此时，s 还不可用
//         let s = "hello"; // 从这一行开始，s 可用了
//                          // 使用 s
//         println!("{s}");
//     } // s 的作用域到此结束
//     // println!("{s}");
// }
