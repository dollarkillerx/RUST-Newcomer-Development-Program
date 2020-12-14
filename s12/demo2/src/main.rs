use std::ops::{Add, Mul, Sub};

fn func_add_org(xs: &mut Vec<i32>, ys: &Vec<i32>) {
    for (x, y) in xs.iter_mut().zip(ys.iter()) {
        *x = Add::add(*x, *y);
    }
}

macro_rules! op {
    ($func: ident, $bound: ident, $method: ident) => {
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>)
        {
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x,*y);
            }
        }
    }
}

op!(func_add,Add,add);
op!(func_mul,Mul,mul);
op!(func_sub,Sub,sub);

fn main() {
    println!("Hello, world!");
    let mut a = vec![1, 2, 3, 4, 5, 6, 7];
    let b = vec![8, 9, 10, 11, 12, 13, 14];

    func_add_org(&mut a, &b);
    println!("{:?}", a);
    println!();

    let mut a1 = vec![1, 2, 3, 4, 5, 6, 7];
    let b1 = vec![8, 9, 10, 11, 12, 13, 14];

    func_add(&mut a1, &b1);
    println!("{:?}", a1);
    println!();

    let mut a1 = vec![1, 2, 3, 4, 5, 6, 7];
    let b1 = vec![8, 9, 10, 11, 12, 13, 14];

    func_mul(&mut a1, &b1);
    println!("{:?}", a1);
    println!();

    let mut a1 = vec![1, 2, 3, 4, 5, 6, 7];
    let b1 = vec![8, 9, 10, 11, 12, 13, 14];

    func_sub(&mut a1, &b1);
    println!("{:?}", a1);
    println!()
}
