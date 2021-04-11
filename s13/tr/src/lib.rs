pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more ...")
    }
}

impl Summary for i32 {
    fn summarize(&self) -> String {
        String::from(format!("summary: {}", self))
    }
}