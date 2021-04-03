const NAME: &'static str = "Demo6";

fn main() {
    println!("{}",NAME);
    let tup: (i32,f64,u8) = (500,6.01,1);
    println!("tup: {} , {}, {}",tup.0,tup.1,tup.2);

    let p: [u8;10] = [1,2,0,0,0,0,0,0,0,0];
    println!("p: {:#?}", p);

    let y = {
        let x = 1;
        x + 3
    };
    print!("y : {}",y);

}
