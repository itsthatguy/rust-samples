mod tools;

use tools::{Hammer, Saw};

fn main() {
    tools::hello();

    let hammer = Hammer {};
    let saw = Saw {};

    Hammer::hi();
    Saw::hi();

    hammer.hello();
    saw.hello();
}

#[cfg(test)]
#[test]
fn struct_instances_work() {
    let hammer = Hammer {};
    assert_eq!(hammer, Hammer {});

    let saw = Saw {};
    assert_eq!(saw, Saw {});
}
