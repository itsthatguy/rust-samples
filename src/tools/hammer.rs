#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hammer {}

impl Hammer {
  pub fn hi() {
    println!("Hi from Hammer");
  }

  pub fn hello(&self) {
    println!("Hello from hammer");
  }
}
