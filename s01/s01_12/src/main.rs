fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y)
}

fn test1() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
}

fn main() {
    test1();
}
