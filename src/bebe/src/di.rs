use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct ComponentRegistration {
    pub type_id: TypeId,
    pub ctor: fn() -> Box<dyn Any>,
}

impl ComponentRegistration {
    pub fn new<T: 'static + Default>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            ctor: || Box::new(T::default()),
        }
    }
}
inventory::collect!(ComponentRegistration);

pub struct Context {
    singletons: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        let mut singletons = HashMap::new();
        for reg in inventory::iter::<ComponentRegistration> {
            singletons.insert(reg.type_id, (reg.ctor)());
        }
        Self { singletons }
    }

    pub fn get<T: 'static>(&self) -> &T {
        self.singletons[&TypeId::of::<T>()]
            .downcast_ref::<T>()
            .unwrap()
    }
}


