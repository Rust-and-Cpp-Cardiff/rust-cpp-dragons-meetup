// struct User {
//     active: bool,
//     username: String,
//     email: String,
// }


// fn main() {
//     println!("Hello, world!");

//     let user1 = User {
//         active: true,
//         username: String::from("Ciara"),
//         email: String::from("Ciara@rustbookclub.com"),
//     };

//     let user2 = User {
//         username: String::from("Ciara"),
//         ..user1
//     };

//     println!("{}", user1.username);
//     println!("{}", user1.active);

//     print_user_active(&user2.active);
//     print_user_active(&user1.active);
// }

// fn print_user_active(user_active: &bool) {
//     println!("{}", user_active);
// }


use std::cmp::max;
use std::cmp::min;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        max(self.width, self.height) >= max(rect.width, rect.height)
        &&
        min(self.width, self.height) >= min(rect.width, rect.height)
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
