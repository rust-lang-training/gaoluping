struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

#[derive(Debug)]
struct Point(f32, f32);

struct AlwaysEqual;

fn main() {
    {
        let user = User {
            active: true,
            username: "John Smith".to_string(),
            email: "j.smith@gmail.com".to_string(),
            sign_in_count: 0,
        };

        let User {
            active,
            username,
            email,
            ..
        } = user;

        println!("active is: {}", active);
        println!("username is: {}", username);
        println!("email is: {}", email);
        println!("sign_in_count is: {}", user.sign_in_count);
    }

    {
        let user = User {
            active: true,
            username: "John Smith".to_string(),
            email: "j.smith@gmail.com".to_string(),
            sign_in_count: 0,
        };
        let user2 = User {
            username: "Lily".to_string(),
            ..user
        };

        println!("user2 username is: {}", user2.username);
    }

    {
        let mut user = User {
            active: true,
            username: "John Smith".to_string(),
            email: "j.smith@gmail.com".to_string(),
            sign_in_count: 0,
        };

        user.sign_in_count = 3;

        println!("sign_in_count is: {}", user.sign_in_count);
    }

    {
        let p = Point(1.0, 2.0);
        println!("point: ({}, {})", p.0, p.1);
        let Point(x, y) = p;
        println!("point: ({}, {})", x, y);
        println!("Point: {:?}", p); // #[derive(Debug)]
    }

    {
        let subject = AlwaysEqual;
    }

    {
        let rectangle1 = Rectangle::square(10f32);
        println!("rectangle1 width: {}", rectangle1.width);

        let mut rectangle2 = Rectangle::new(3.0, 4.0);
        rectangle2.scale(1f32, 2f32);
        println!("rectangle2 width: {}", rectangle2.width);
        println!("perimeter {}", rectangle2.perimeter());
        println!("area {}", rectangle2.area());
    }
    

    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(i32, i32, i32)
        }

        let q = Message::Quit;
        let m = Message::Move {x: 100, y: 100};
        let n = Message::Write(String::from("hello message"));
        let c = Message::ChangeColor(123, 5, 101);
        println!("q: {:?}", q);
        println!("q: {:?}", m);
        println!("q: {:?}", n);
        println!("q: {:?}", c);
    }

}


struct Rectangle {
    width: f32,
    height: f32
}

impl Rectangle {
    fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
    fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height
        }
    }

    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0f32
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn scale(&mut self, width_scale: f32, height_scale: f32) {
        self.width = self.width * width_scale;
        self.height = self.height * height_scale;
    }
}

// 结构体可以有多个impl块
impl Rectangle {

}

// Rust 中的结构体不支持继承

