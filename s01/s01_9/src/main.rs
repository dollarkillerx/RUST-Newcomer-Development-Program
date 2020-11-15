use factory::people;
use hello;

mod factory {
    pub mod people {
        pub fn hello_world() {
            println!("Hello World");
        }
    }
}

fn main() {
    people::hello_world();
    hello::hello_world();
    hello::hp::pkg::hello_world();
}
