use std::marker::PhantomPinned;
use std::pin::{self, Pin};

fn learning_pin() {
    let data = Box::pin(5);
    let mut value = 8;
    let ptr = Pin::new(&mut value);
}

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test { a: String::from(txt), b: std::ptr::null() }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}
#[test]
pub fn test_self_ref() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    std::mem::swap(&mut test1, &mut test2);
    test1.a = "I've totally changed now!".to_string();
    println!("a: {}, b: {}", test1.a(), test1.b());
    println!("a: {}, b: {}", test2.a(), test2.b());
}

struct Dog;

fn speak(dog: Dog) {
    println!(" {:p} says Woof!", &dog as *const Dog);
}

fn eat(food: String) {
    println!("I'm eating {} at {:p} !", food, &food as *const String)
}

#[test]
pub fn test_move() {
    let dog = Dog;
    println!(" {:p} says Woof!", &dog as *const Dog);
    let dog2 = dog;
    println!(" {:p} says Woof!", &dog2 as *const Dog);

    let food = String::from("banana");
    println!("I'm eating {} at {:p} !", food, &food as *const String);
    eat(food);

    let food = Box::new(String::from("apple"));
    println!("{:p}", &food as *const Box<String>);
    println!("I'm eating {} at {:p} !", *food, &*food as *const String);
    let food2 = food;
    println!("{:p}", &food2 as *const Box<String>);
    println!("I'm eating {} at {:p} !", *food2, &*food2 as *const String);
}
