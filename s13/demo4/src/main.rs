fn annotate<T, F>(f: F) -> F
    where for<'a> F: Fn(&'a T) -> &'a T {
    f
}

fn annotate2<'a, T: 'a, F>(f: F) -> F
    where F: Fn(&'a T) -> &'a T {
    f
}

fn main() {
    println!("Hello, world!");
    // let f = |x: &i32| x;
    // let f: for<'a> Fn(&'a i32) -> &'a i32 = |x| x;
    let f = annotate(|x: &i32| x);
    let f = annotate2(|x: &i32| x);
    let i = &3;
    let j = f(i);
}
