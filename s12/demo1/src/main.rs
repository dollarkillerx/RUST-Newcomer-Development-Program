use std::collections::HashMap;
macro_rules! hey {
    () => { println!("hi, nice to meet you!"); };
    ($xname: expr) => { println!("say hi to {}", $xname); };
    ($xname: expr, $yname: expr) => { println!("{} say hi to {}",$xname, $yname); };
    // ( $($name: expr),* ) => {
    //     {
    //         let mut sc = Vec::new();
    //         $(
    //             sc.push($name);
    //         )*
    //         println!("{:?}",$name);
    //     }
    // };
}

// item：条目，例如函数、结构、模块等
// block：代码块
// stmt：语句
// pat：模式
// expr：表达式
// ty：类型
// ident：标识符
// path：路径，例如 foo、 ::std::mem::replace, transmute::<_, int>, ...
// meta：元信息条目，例如 #[...]和 #![rust macro...] 属性
// tt：词条树

// macro_rules! macron_name {
//     (pattern1!) => { code blocks ;};
//     (pattern2!) => { code blocks ;};
// }

macro_rules! vxc {
    ( $( $name: expr ),* ) => {
        {
            let mut vec = Vec::new();
            $( vec.push($name); )*
            vec
        }
    };
}

// * 零次或者多次  + 一次或者多次

macro_rules! map {
    ( $($key: expr => $val: expr),+ ) => {
        {
            let mut mp = HashMap::new();
            $(
                mp.insert($key, $val);
            )+
            mp
        }
    }
}


fn main() {
    println!("Hello, world!");

    hey!();
    hey!("dollarkiller");
    hey!("dollarkiller", "worldlink");


    let p = vxc!(1,2,3,4,5,5,5);
    println!("{:?}",p);

    let m = map!(
        "name" => "dollarkiller",
        "age" => "20"
    );

    println!("{:?}",m);
}
