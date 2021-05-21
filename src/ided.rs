// this is if you didnt care about object making a call, just something inside that object
// like for example the id
// traits have no idea about any ot fns or vars on the thing that called it.
pub trait Ided {
  fn my_id(&self) -> i64;
}
