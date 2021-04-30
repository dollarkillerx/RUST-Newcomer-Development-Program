mod link_v1;
mod link_v2;

mod test {
    use super::*;

    #[test]
    fn test_v1() {
        let mut c = link_v1::List::new();
        for i in 0..35 {
            c.push(i);
        }

        while let Some(cs) = c.pop() {
            println!("cs: {}", cs);
        }
    }

    #[test]
    fn test_v2() {
        let mut c = link_v2::List::new();
        for i in 0..35 {
            c.push(i);
        }

        while let Some(cs) = c.pop() {
            println!("cs: {}", cs);
        }
    }
}