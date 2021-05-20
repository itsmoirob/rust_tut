use parking_lot::Mutex;
use std::{error::Error, sync::Arc};

// #[test]
// fn my_test() {
//     assert!(true)
// }

// how to make a function
fn greet_int(x: i64) {
    println!("Hello to number {}.", x);
}

// how to make a string function
fn greet_str(x: &str) {
    println!("Hello to {}.", x);
}

// how to return something
fn return_str(x: &str) -> String {
    format!("Get back {}.", x)
}

// using struct
#[derive(Debug)]
struct Employee {
    name: String,
    id: i64,
}

impl Employee {
    fn new(name: String, id: i64) -> Employee {
        Employee { name, id }
    }

    fn id(&self) -> i64 {
        self.id
    }

    // "&" means "to borrow" or reference, not own.
    // "String"s require extra steps to be able to pass it value.
    // Unlike i64s
    // so use clone()
    fn name(&self) -> String {
        self.name.clone()
    }

    // this is same as above
    // only here rather that two pointer pointing at separate items
    // here two pointers point to same thing
    // fn clone() is better for beginners.
    // 'a means "a type of lifetime"
    // could be 'a 'b 'c 'd 'T
    fn name_lifetime<'a>(&'a self) -> &'a str {
        &self.name
    }
}

fn borrow_thing(employee: &Employee) {
    println!("I have borrowed this {}.", employee.name);
}

// using enum
#[derive(Debug)]
enum IsTrue {
    True(i64),
    False,
}

fn process() -> Result<String, Box<dyn Error>> {
    Ok(String::from("Yay!"))
}

fn main() -> Result<(), Box<dyn Error>> {
    // join a string
    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // use functions
    let my_name = "Arthur";
    greet_int(100);
    greet_str(my_name);
    println!("{}", return_str(my_name));

    // with just struct
    let employee_struct = Employee {
        name: "Arthur_struct".to_string(),
        id: 1,
    };
    println!(
        "Hello employee_struct {} with id {}.",
        employee_struct.name, employee_struct.id
    );
    println!("{:?}", employee_struct); // {:?} allows to debug is derive
    println!("{:#?}", employee_struct); // {:?} as above but pretty print

    // with impl
    let employee_impl = Employee::new(String::from("Arthur_impl"), 200);

    borrow_thing(&employee_impl); // & is allowing to borrow employee for borrow_thing fn

    println!(
        "Hello employee_impl {} with id {}.",
        employee_impl.name, employee_impl.id
    );
    println!(
        "Hello employee_impl {} with id {}.",
        employee_impl.name(),
        employee_impl.id()
    );
    println!(
        "Hello employee_impl {} with id {}.",
        employee_impl.name_lifetime(),
        employee_impl.id()
    );
    println!("{:?}", employee_impl); // {:?} allows to debug is derive
    println!("{:#?}", employee_impl); // {:?} as above but pretty print

    let my_value_true = IsTrue::True(100);
    let my_value_false = IsTrue::False;
    println!("{:?}", my_value_true);
    println!("{:?}", my_value_false);
    match my_value_true {
        IsTrue::True(x) => println!("True! with {}", x),
        IsTrue::False => println!("False!"),
    }

    // with impl
    let employee_mutex_lock = Employee::new(String::from("Arthur_mutex_lock"), 300);
    // thread safe version of shared item. Perhaps something really big
    // put lock on . needs to be unlocked
    let locked_employee = Mutex::new(employee_mutex_lock);
    let unlock_employee = locked_employee.lock();
    println!(
        "Hello unlock_employee {} with id {}.",
        unlock_employee.name, unlock_employee.id
    );

    // OR

    // Using Atomic Reference Count Arc
    let employee_arc_lock = Employee::new(String::from("Arthur_arc_lock"), 400);
    // thread safe version of shared employee
    // put lock on . needs to be unlocked
    let reference_counted = Arc::new(employee_arc_lock);

    // here the "?" this is like try catch.
    let result = process();
    // match result {
    //     Ok(x) => {
    //         println!("{}", x);
    //     }
    //     Err(_) => {
    //         println!("It errored");
    //     }
    // }

    // OR
    println!("{}", result?);

    // OR
    println!("{}", process()?);

    Ok(())
}
