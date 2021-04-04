use itertools::Itertools;

fn main() {
    println!("Hello, world!");


    for i in (0..21).filter(|x| (x % 2 == 0)) {
        print!("{} ", i);
    }
    println!();

    for i in (1..11).map(|x| (x * x)) {
        print!("{} ", i);
    }
    println!();

    let cities = ["Toronto", "New York", "Melbourne"];
    for city in cities.iter() {
        print!("{}, ", city);
    }
    println!();

    let c = 0..cities.len();
    let matrix = cities.iter().zip(c.into_iter());
    for (k, v) in matrix {
        println!("k: {} v : {}", v, k);
    }
    println!();

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let nums_2 = nums.clone();
    let mum = nums_2.iter().map(|x| x * 2).collect::<Vec<i32>>();
    // collect()是一个普通的消费者。 它从一个迭代器获取值并将它们转换为所需类型的集合
    // let mum = nums_2.iter().map(|x| x * 2);
    for i in &mut nums {
        // print!("{} ,", i);
        *i = *i * 2;
    }
    println!("{:?}", nums);
    println!("{:?}", mum);

    let v = (1..11).collect::<Vec<i32>>();
    for (i, n) in v.iter().enumerate() {
        println!("k: {}, v : {}", i, n);
    }

    let v = (1..)
        .map(|x| x * x)
        .filter(|x| x % 5 == 0)
        .take(10) // => limit 10
        .collect::<Vec<i32>>();
    println!("{:?}", v);

    let creatures = vec!["banshee", "basilisk", "centaur"];
    let list = creatures.iter().join(", ");
    println!("list: {}", list);

    let l2 = vec![1,2,3,1,2,3,12,45,12,32,16,1,4,5,63,2,1,40];
    let unique = l2.iter().unique();
    println!("unique: {:?}", unique);
}
