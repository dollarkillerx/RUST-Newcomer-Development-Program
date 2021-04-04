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

    let m = if true {6} else {7};
    print!("m: {}",m);

    for i in 0..=6 {
        println!("i: {}",i);
    }

    let mut a = [10,20,20,30,60,70];
    for i in a.iter_mut() {
        if i == 20 {
            
        }
        println!("i: {}", i);
    }
}

// fn pp(s: str...) {
//     println!()
// }