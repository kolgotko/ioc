use std::ops::DerefMut;
use rand::{
    thread_rng,
    SeedableRng,
    rngs::{ThreadRng, StdRng},
};
use ioc::{Container, Resolvable, resolvable};
use std::ops::{Deref};

#[derive(Clone, Debug)]
struct Rng<T>(T);

#[resolvable(new)]
impl Rng<ThreadRng> {
    fn new() -> Self {
        Self(thread_rng())
    }
}

#[resolvable(new)]
impl Rng<StdRng> {
    fn new() -> Self {
        Self(StdRng::from_entropy())
    }
}

impl<T> Deref for Rng<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Rng <T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! inject {
    ($type:ident) => { Rng<$type> };
}

#[derive(Clone)]
struct Test {
    rng: ThreadRng,
}

#[resolvable(new)]
impl Test {
    fn new(rng: inject!(ThreadRng)) -> Self {
        Self { rng: *rng }
    }
}

fn main() {

    let mut container = Container::default();
    let test: Test = container.try_resolve().unwrap();

}
