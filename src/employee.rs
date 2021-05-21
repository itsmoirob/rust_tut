use crate::ided::Ided;

// using struct
#[derive(Debug)]
pub struct Employee {
  pub name: String,
  pub id: i64,
}

impl Employee {
  pub fn new(name: String, id: i64) -> Employee {
    Employee { name, id }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  // "&" means "to borrow" or reference, not own.
  // "String"s require extra steps to be able to pass it value.
  // Unlike i64s
  // so use clone()
  pub fn name(&self) -> String {
    self.name.clone()
  }

  // this is same as above
  // only here rather that two pointer pointing at separate items
  // here two pointers point to same thing
  // fn clone() is better for beginners.
  // 'a means "a type of lifetime"
  // could be 'a 'b 'c 'd 'T
  pub fn name_lifetime<'a>(&'a self) -> &'a str {
    &self.name
  }
}

impl Ided for Employee {
  fn my_id(&self) -> i64 {
    self.id()
  }
}

fn use_ided_impl(x: impl Ided) {
  println!("x id is {}", x.my_id());
  // use like use_ided(employee)
}

fn use_ided_dyn(x: Box<dyn Ided>) {
  println!("x id is {}", x.my_id());
  // use like use_ided(Box::new(employee))
}

pub fn borrow_thing(employee: &Employee) {
  println!("I have borrowed this {}.", employee.name);
}
