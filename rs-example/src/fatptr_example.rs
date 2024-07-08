#![allow(clippy::size_of_ref)]

// A reference to a trait object is a fat pointer: (data_ptr, vtable_ptr)
use std::fmt::Debug;
use std::mem::size_of;

trait SomeTrait {}

fn display_ptr_len() {
    println!("======== The size of different pointers in Rust: ========");
    println!("&dyn Trait:-----{}", size_of::<&dyn SomeTrait>());
    println!("&[&dyn Trait]:--{}", size_of::<&[&dyn SomeTrait]>());
    println!("Box<Trait>:-----{}", size_of::<Box<dyn SomeTrait>>());
    println!("&i32:-----------{}", size_of::<&i32>());
    println!("&[i32]:---------{}", size_of::<&[i32]>());
    println!("Box<i32>:-------{}", size_of::<Box<i32>>());
    println!("&Box<i32>:------{}", size_of::<&Box<i32>>());
    println!("[&dyn Trait;4]:-{}", size_of::<[&dyn SomeTrait; 4]>());
    println!("[i32;4]:--------{}", size_of::<[i32; 4]>());
    println!("&Vec<usize>:--------{}", size_of::<Vec<usize>>());
}

trait Test {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn mul(&self) -> i32;
}

// This will represent our home brewn fat pointer to a trait object
#[repr(C)]
struct FatPointer<'a> {
    /// A reference is a pointer to an instantiated `Data` instance
    data: &'a mut Data,
    /// Since we need to pass in literal values like length and alignment it's
    /// easiest for us to convert pointers to usize-integers instead of the other way around.
    vtable: *const usize,
}

// This is the data in our trait object. It's just two numbers we want to operate on.
struct Data {
    a: i32,
    b: i32,
}

// ====== function definitions ======
fn add(s: &Data) -> i32 {
    println!("length -- {}", std::mem::size_of_val(&s));
    s.a + s.b
}
fn sub(s: &Data) -> i32 {
    s.a - s.b
}
fn mul(s: &Data) -> i32 {
    s.a * s.b
}

fn max(s: &Data) -> i32 {
    if s.a > s.b {
        s.a
    } else {
        s.b
    }
}

fn xxx(d: impl Test) {
    println!("-----------> {}", std::mem::size_of_val(&d));
}

#[test]
fn make_fat_pointer() {
    let mut data = Data { a: 3, b: 2 };
    // vtable is like special purpose array of pointer-length types with a fixed
    // format where the three first values has a special meaning like the
    // length of the array is encoded in the array itself as the second value.
    let vtable = vec![
        0, // pointer to `Drop` (which we're not implementing here)
        6, // lenght of vtable
        8, // alignment
        // we need to make sure we add these in the same order as defined in the Trait.
        add as usize, // function pointer - try changing the order of `add`
        sub as usize, // function pointer - and `sub` to see what happens
        mul as usize, // function pointer
                      // max as usize, // function pointer
    ];

    let fat_pointer = FatPointer { data: &mut data, vtable: vtable.as_ptr() };
    let test = unsafe { std::mem::transmute::<FatPointer, &dyn Test>(fat_pointer) };
    println!("length of test: {}", std::mem::size_of_val(&test));
    println!("length of data: {}", std::mem::size_of_val(&data));

    // And voalá, it's now a trait object we can call methods on
    println!("Add: 3 + 2 = {}", test.add());
    println!("Sub: 3 - 2 = {}", test.sub());
    println!("Mul: 3 * 2 = {}", test.mul());
}

#[test]
fn test_display_ptr_len() {
    display_ptr_len();
}

trait Animal {
    fn speak(&self);
}

#[derive(Debug)]
struct Dog;

impl Dog {
    fn eat(&self) {
        println!("Nom nom nom");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn display_sth1<T: Debug>(sth: T) {
    println!("1. {:?}, length: {}", sth, std::mem::size_of_val(&sth));
}

fn display_sth2(sth: impl Debug) {
    println!("2. {:?}, length: {}", sth, std::mem::size_of_val(&sth));
}

fn display_sth3(sth: &dyn Debug) {
    let x = sth;
    let y = &sth;
    // 这里是 &dyn Debug 类型的大小，实际是个胖指针的大小
    println!("3. {:?}, length: {}", sth, std::mem::size_of_val(&sth));
    // 这里是 dyn Debug 类型的大小，实际是具体数据类型的大小
    println!("3. {:?}, length: {}", sth, std::mem::size_of_val(sth));
}

trait Transform {
    fn transform(&self) -> Self;
}

#[derive(Debug)]
struct Circle(u32);

impl Transform for Circle {
    fn transform(&self) -> Self {
        Circle(self.0 * 2)
    }
}

#[test]
fn test_display_trait_object() {
    let dog = Dog;
    display_sth1(dog);
    let dog = Dog;
    display_sth2(dog);
    let dog = Dog;
    display_sth3(&dog);

    let shape = Circle(5);
    display_sth3(&shape);
}

#[test]
fn display_dyn_trait_fat_pointer() {
    let dog: Box<dyn Animal> = Box::new(Dog);
    println!("dog size: {}", std::mem::size_of_val(&dog));

    println!("dog size: 0x{:X}", Dog::speak as usize);

    // 将 trait 对象转换为胖指针
    let (data_ptr, vtable_ptr) = unsafe { std::mem::transmute::<_, (usize, usize)>(dog) };
    let vtable = unsafe { &*(vtable_ptr as *const [usize; 3]) };
    println!("vtable: {:?}", vtable);

    // data_ptr.eat();
    // data_ptr.speak();

    let v: Vec<u64> = vec![1, 2, 3, 4];
    let c: Box<dyn Debug> = Box::new(v);

    let (_, vtable) = unsafe { std::mem::transmute::<_, (usize, usize)>(c) };
    println!("{:?}", unsafe { &*(vtable as *const [usize; 3]) });
}
