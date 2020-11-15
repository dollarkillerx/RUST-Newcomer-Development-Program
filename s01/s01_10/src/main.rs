use s01_10::*;

fn function() {
    println!("this is 1 function");
}

mod cool {
    pub fn function() {
        println!("this is cool function");
    }
}

mod my {
    fn function() {
        println!("this is my function");
    }

    mod cool {
        pub fn function() {
            println!("my::cool::function");
        }
    }

    pub fn indirect_call() {
        self::function();
        function();

        self::cool::function();

        super::function();

        {
            // create 根目录
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();

    hello::hello_world();
}
