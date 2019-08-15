use std::ops::DerefMut;
use std::ops::Deref;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc};
pub use resolve_macro::{resolvable, resolvable_via_default};

#[derive(Clone)]
pub struct ResolvableWrap<T>(T);

impl<T> Deref for ResolvableWrap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ResolvableWrap <T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[macro_export]
macro_rules! resolve {
    ($type:ident) => { ResolvableWrap<$type> };
}

pub trait Resolvable {
    fn resolve(container: &Container) -> Self;
}

pub enum Dependency {
    Entity(Arc<dyn Any>),
    Factory(Arc<dyn Any>),
}

#[derive(Default)]
pub struct Container(HashMap<TypeId, Dependency>);

impl Container {
    pub fn add_factory<T, F>(&mut self, factory: F) -> ()
    where
        F: Fn(&Container) -> T + 'static,
        T: 'static + Clone + Resolvable,
    {
        let key = TypeId::of::<T>();
        let factory: Box<dyn Fn(&Container) -> T> = Box::new(factory);
        self.0.insert(key, Dependency::Factory(Arc::new(factory)));
    }

    pub fn add<T: 'static + Clone + Resolvable>(&mut self, entity: T) {
        let key = TypeId::of::<T>();
        self.0.insert(key, Dependency::Entity(Arc::new(entity)));
    }

    pub fn try_resolve<T: 'static + Clone + Resolvable>(&self) -> Option<T> {
        let key = TypeId::of::<T>();
        let dependency = self.0.get(&key);

        if dependency.is_none() {
            Some(T::resolve(self))
        } else {
            match self.0.get(&key)? {
                Dependency::Entity(any) => {
                    let entity = any.downcast_ref::<T>()?.clone();
                    Some(entity)
                },
                Dependency::Factory(any) => {
                    let factory = any.downcast_ref::<Box<dyn Fn(&Container) -> T>>()?;
                    Some(factory(self))
                },
            }
        }

    }
}
