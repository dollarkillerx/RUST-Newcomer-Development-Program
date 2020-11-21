use std::collections::HashMap;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

fn main() {
    println!("Hello, world!");
    test_once_cell();
    test_lazy_static();
}

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(14, "Spica13".to_string());
    m.insert(15, "Spica15".to_string());
    Mutex::new(m)
});

// set(T) => 安全的设置全局变量
// get() -> T => 获取已经设置的全局变量
// get_or_init(Fn) => 获取全局变量，为空则初始化
// Lazy::new(Fn) => 延迟创建全局变量

fn global_data() -> &'static Mutex<HashMap<i32, String>> {
    static INSTANCE: OnceCell<Mutex<HashMap<i32, String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(13, "Spica".to_string());
        m.insert(74, "Hoyten".to_string());
        Mutex::new(m)
    })
}

fn test_once_cell() {  // 是一种只执行一次的容器，多用于全局变量，安全初始化，或者延迟初始化
    let p = GLOBAL_DATA.lock().unwrap();
    println!("{:?}", p.get(&13));
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<u32, String>> = {
        let mut m = HashMap::new();
        m.insert(0,"foo".to_string());
        m.insert(1,"bar".to_string());
        Mutex::new(m)
    };
}

fn test_lazy_static() {
    println!("{:?}",HASHMAP.lock());
}