use parking_lot::Mutex;
use std::{error::Error, sync::Arc};

mod employee;
use employee::{borrow_thing, Employee};

mod ided;

mod lib;
use lib::*;

// #[test]
// fn my_test() {
//     assert!(true)
// }

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

    // ///////////////////////////////

    let mut employee_mutable = Employee::new(String::from("Arthur_emutable"), 500);

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
