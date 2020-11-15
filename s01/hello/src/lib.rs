pub mod hp;

pub fn hello_world() {
    println!("Hello World2!!!")
}

#[cfg(test)]
mod tests {
    use crate::hp;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn hello_world_test() {
        crate::hello_world();
        hp::pkg::hello_world();
    }
}
