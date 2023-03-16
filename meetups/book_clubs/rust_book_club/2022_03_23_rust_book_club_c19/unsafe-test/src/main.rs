
extern "C" { // The ABI of the foreign function
    fn abs(input: i32) -> i32;
}
    
        
fn main() {
    println!("running...");

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));                
    }

    let mut val = MyIterable{};

    // println!("{:?}", val.next()); // ERROR if more than one next definition
    println!("{:?}", <MyIterable as MyIterator<f32>>::next(&mut val));
    println!("{:?}", <MyIterable as MyIterator<u32>>::next(&mut val));

    let prog = Programming{};
    prog.heart_print();

    let mut my_vec = MyWrapper(vec!["sort".to_string(),"hello".to_string(),"world".to_string(),"string".to_string()]);
    my_vec.0.sort();

    println!("{}", my_vec);

    let mut people = People::new();
    people.add_person("Bob");

    let _hello: &str = "hello";

    let list_of_statuses_func: Vec<Status> = (0u32..20).map(Status::Value).collect();

    let list_of_statuses_clos: Vec<Status> = (0u32..20).map(|val| Status::Value(val)).collect();

    let clos = returns_closure();
    clos(5);


}

enum Status {
    Value(u32),
    Stop,
}


pub trait MyIterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct MyIterable{}

impl MyIterator<u32> for MyIterable {
    fn next(&mut self) -> Option<u32>{
        return Some(1);
    }
}

impl MyIterator<f32> for MyIterable {
    fn next(&mut self) -> Option<f32>{
        return Some(2.2);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl MyAdd<Point> for Point {
    type Output = Point;

    fn my_add(self, other : Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait MyAdd<Rhs> {
    type Output;

    fn my_add(self, rhs: Rhs) -> Self::Output;
}

use std::{fmt, collections::HashMap};
trait HeartPrint: fmt::Display + Loveable {
    fn heart_print(&self) {
        let output = self.to_string(); // to_string() is on fmt::Display trait
        println!("❤️ {}❤️", output);
    }
}

trait Loveable{}

#[derive(Debug)]
struct Programming{}

impl fmt::Display for Programming {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Programming")
    }
}

impl Loveable for Programming {}
impl HeartPrint for Programming {}


struct MyWrapper(Vec<String>);


impl fmt::Display for MyWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn new_id() -> i32{
    return 0;
}

struct People(HashMap<i32, String>);

impl People {
    fn new() -> People {
        People(HashMap::new())
    }

    fn add_person(&mut self, name: &str) {
        self.0.insert(new_id(), name.to_string());
    }
}

fn returns_func() -> fn() -> i32 {
    new_id
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |i| i + 1
}