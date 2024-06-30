use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    
    // {
    //     let s: Rc<String> = Rc::new(String::from("shirataki"));
    //     let t: Rc<String> = s.clone();
    //     let u: Rc<String> = s.clone();

    //     println!("ref count: {}", Rc::strong_count(&s));
    //     println!("ref count: {}", Rc::strong_count(&t));
    //     println!("ref count: {}", Rc::strong_count(&u));
    // }


    {
        let s: Rc<String> = Rc::new(String::from("shirataki"));
        println!("ref count: {}", Rc::strong_count(&s));

        {
            let t: Rc<String> = s.clone();
            println!("ref count: {}", Rc::strong_count(&t));
        }
        
        let u: Rc<String> = s.clone();
        println!("ref count: {}", Rc::strong_count(&s));
        println!("ref count: {}", Rc::strong_count(&u));
    }

    {
        // Box<T> 用来将值存储到堆上
        let b = Box::new(5);
        println!("b = {}", b);
    }
    
    {
        let s = RefCell::new(String::from("Hello world"));
        append_string(&s);
        println!("s is: {}", s.borrow());

        fn append_string(s: &RefCell<String>) {
            let mut ms = s.borrow_mut();
            (*ms).push_str(" I'm Rust");
        }
    }

    {
        let s = RefCell::new(String::from("Hello world"));
        append_string(&s);
        println!("s is: {}", s.borrow());

        fn append_string(s: &RefCell<String>) {
            let rs = s.borrow();
            let mut ms = s.borrow_mut();
            (*ms).push_str(" I'm Rust");
            println!("rs: {}", rs);
        }
    }
}
