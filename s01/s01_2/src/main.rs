fn main() {
    tup();
    slice();
}

fn tup() {
    let tup = (1, "name", 12);
    let (a, b, c) = tup;
    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn slice() {
    let xs: [i32; 5] = [0, 1, 2, 3, 4];
    let ys: [i32; 100] = [0; 100];
    println!("xs len: {}", xs.len());

    let b = &ys[0..3];
    println!("{:?}", b);

    let a = &ys[..];
    println!("{:?}", a);
}