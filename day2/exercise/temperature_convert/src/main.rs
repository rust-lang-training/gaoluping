use std::io;

fn main() {
    loop {
        println!("摄氏度转华氏度，输入1");
        println!("华氏度转摄氏度，输入2");
        println!("退出输入0");
        let mut choise = String::new();
        io::stdin().read_line(&mut choise).expect("Fail to read line");
        let choise = choise.trim();

        if choise == "0" {
            break;
        }

        if choise == "1" {
            println!("请输入摄氏度");
            let mut c = String::new();
            io::stdin().read_line(&mut c).expect("Fail to read line");
            let c: f32 = c.trim().parse().expect("请输入数字");
            let f: f32 = c_to_f(c);
            println!("摄氏度: {}, -> 华氏度: {:.2}", c, f);
        }

        if choise == "2" {
            println!("请输入华氏度");
            let mut f = String::new();
            io::stdin().read_line(&mut f).expect("Fail to read line");
            let f: f32 = f.trim().parse().expect("请输入数字");
            let c: f32 = f_to_c(f);
            println!("华氏度: {}, -> 摄氏度: {:.2}", f, c);
        }
    }
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn c_to_f(c: f32) -> f32 {
    (c * 1.8) + 32.0
}
