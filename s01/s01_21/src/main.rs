trait Animal {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("旺旺...");
    }
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("喵喵...");
    }
}

fn animal_speak<T: Animal>(animal: T) {
    animal.speak();
}

fn animal_speak2(animal: &dyn Animal) {
    animal.speak();
}

fn test1() {
    let dog = Dog;
    let cat = Cat;

    animal_speak(dog);
    animal_speak2(&cat);
}

fn main() {
    test1();

    test2();
}

trait Action {
    fn act(&self);
}

struct DefaultAction {
    content: String,
}

impl Action for DefaultAction {
    fn act(&self) {
        println!("do {}", self.content);
    }
}

struct User<'a> {
    action: &'a dyn Action,
}

impl<'a> User<'a> {
    pub fn user_act(&self) {
        self.action.act()
    }
}

fn test2() {
    let action = DefaultAction {
        content: "hello Rust".to_string(),
    };
    let user = User {
        action: &action,
    };
    user.user_act();
}

// trait Action {
//     fn act(&self);
// }
//
// struct DefaultAction {
//     content: String,
// }
//
// impl Action for DefaultAction {
//     fn act(&self) {
//         println!("do {}", self.content);
//     }
// }
//
// struct User<T> {
//     action: T,
// }
//
// impl<T: Action> User<T> {
//     pub fn user_act(&self) {
//         self.action.act()
//     }
// }
//
// fn test2() {
//     let action = DefaultAction {
//         content: "hello Rust".to_string(),
//     };
//     let user = User {
//         action,
//     };
//     user.user_act();
// }