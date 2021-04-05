use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() { // g更具空格切割 str iter
        println!("{}", word);
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn test2() {
    let mut scores = HashMap::new();
    scores.insert("hello".to_string(), 123456);
    scores.insert("hello1".to_string(), 123456);
    scores.insert("hello2".to_string(), 123456);

    for (k, v) in scores.iter() {
        println!("k: {}, v : {}", k, v);
    }

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
    println!("scores = {:?}", scores);
    println!("teams: {:?}", teams);

    for (k, v) in &mut scores {
        if *k == "Blue" {
            *v = &600
        }
    }
    println!("scores = {:?}", scores);
    let p = String::from("bbc");
    scores.entry(&p).or_insert(&60);
    println!("scores = {:?}", scores);
}

fn test1() {
    println!("Hello, world!");

    let mut p = vec!["h1", "h2", "h3"];
    p.push("h4");
    println!("p: {:?}", p);

    for (k, v) in p.iter().enumerate() {
        println!("K: {} V: {}", k, v);
    }

    let mut s2 = vec![String::from("hello"), String::from("world")];

    // for i in &mut s2 {
    //     i.push_str(" world");
    // }

    // for (k, v) in s2.iter().enumerate() {
    //     if k == 0 {
    //         s2.insert(k, v.to_string() + " world")
    //     }
    //     s2.insert(k, v.to_string() + " hello")
    // }
    //
    // println!("s2: {:?}", s2);

    for i in 0..s2.len() {
        if i == 0 {
            s2.get_mut(i).unwrap().push_str(" world");
            continue;
        }

        s2.get_mut(i).unwrap().push_str(" hello");
    }

    println!("s2: {:?}", s2);


    let ss1 = String::from("Hello");
    let ss2 = String::from(" world");
    let s3 = ss1 + &ss2;
    // println!("s1: {}", ss1);
    println!("s2: {}", ss2);

    drop(ss2);
    // println!("s2: {}", ss2); value borrowed here after move
    println!("s3: {}", s3);

    let s1 = "我爱你";
    println!("s1 len: {}", s1.len());
    println!("s1 len: {}", s1.as_bytes().len());
    // println!("s1 len: {}", s1.chars().len());
}