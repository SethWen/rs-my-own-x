#[derive(rs_macro::HelloDerive)]
struct MyStruct;

trait HelloWorld {
    fn hello_world();
}

#[test]
pub fn test_hello_world() {
    MyStruct::hello_world();
}

#[rs_macro::hello_attr]
fn hello_attr_macro() {
    println!("Hello from my_function!");
}

#[test]
pub fn test_hello_attr_macro() {
    _hello_attr_macro();
}

#[test]
pub fn test_add() {
    let sum = rs_macro::add!(2 + 3);
    println!("sum = {}", sum);
}
