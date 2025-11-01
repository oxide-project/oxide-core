use std::task::Context;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::OnceLock,
};

pub struct ApplicationContext {
    beans: HashMap<TypeId, BeanDefinition>,
}

type FactoryFn = fn(&ApplicationContext) -> Box<dyn Any>;

struct BeanDefinition {
    factory: FactoryFn,
    instance: OnceLock<&'static dyn Any>,
}

impl ApplicationContext {
    pub fn new() -> Self {
        Self {
            beans: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, factory: FactoryFn) {
        self.beans.insert(
            TypeId::of::<T>(),
            BeanDefinition {
                factory,
                instance: OnceLock::new(),
            },
        );
    }

    pub fn get<T: 'static>(&self) -> &'static T {
        let def = self
            .beans
            .get(&TypeId::of::<T>())
            .expect(&format!("Bean {} not found", std::any::type_name::<T>()));

        def.instance
            .get_or_init(|| {
                // 1. создаём Box<dyn Any>
                let boxed_any = (def.factory)(self);
                // 2. превращаем в &'static dyn Any
                Box::leak(boxed_any)
            })
            .downcast_ref::<T>()
            .expect(&format!(
                "Bean registered under type {}, but type mismatch occurred",
                std::any::type_name::<T>()
            ))
    }
}

struct SomeConfig;

#[bebe::config]
impl SomeConfig{
    
}
fn main() {

}