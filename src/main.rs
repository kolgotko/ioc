use std::error::Error;
use resolve_macro::{ resolvable };
use ioc::{Resolvable, Container};

#[derive(Debug, Clone, Default)]
struct Test(i32);

#[resolvable(new)]
impl Test {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Clone)]
struct Foo {
    test: Test,
}

#[resolvable(new)]
impl Foo {
    fn new(test: Test) -> Self {
        Self { test }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let mut container = Container::default();

    container.add_factory(|_| Test(123));
    container.add(Test(456));

    let foo: Foo = container.try_resolve().unwrap();
    dbg!(foo);

    Ok(())
}
