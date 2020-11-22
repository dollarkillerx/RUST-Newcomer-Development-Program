use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<HashMap<String, i32>> = {
        let db = HashMap::new();
        Mutex::new(db)
    };
}