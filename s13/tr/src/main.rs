use tr::Summary;

fn main() {
    println!("Hello, world!");

    let w = Weather { weather: "sss".to_string() };
    println!("summary: {}", w.summarize());

    let c = 12;
    println!("c: {}", c.summarize());

    notify(c);
    notify2(c);
}

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

struct Weather {
    weather: String
}

impl Summary for Weather {
    // overwrite
    fn summarize(&self) -> String {
        self.weather.clone()
    }
}

fn notify(item: impl Summary) {
    println!("notify: {}", item.summarize());
}

fn notify2<T: Summary>(item: T) {
    println!("notify2: {}", item.summarize());
}