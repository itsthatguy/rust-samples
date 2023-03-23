#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Saw {}

impl Saw {
  pub fn hi() {
    println!("Hi from Saw");
  }
  pub fn hello(&self) {
    println!("Hello from saw");
  }
}
