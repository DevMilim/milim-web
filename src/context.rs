use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
#[derive(Debug)]
pub struct RequestContext {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl RequestContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.data
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut())
    }
    pub fn data<T: Send + Sync + 'static>(&mut self, val: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(val));
    }
}

/// Sera utilizado para obter e registrar estados
#[derive(Debug)]
pub struct Context {
    pub(crate) map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Context {
    pub fn state<T: Send + Sync + 'static>(&mut self, val: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(val));
    }
    pub fn get_state<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }
}

fn downcast_owned<T: 'static>(boxed: Box<dyn Any>) -> Option<T> {
    boxed.downcast().ok().map(|boxed| *boxed)
}
