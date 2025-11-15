use std::{
    any::{Any, TypeId},
    collections::HashMap,
    io::Sink,
    sync::{Arc, RwLock},
};
/// Sera utilizado para obter e registrar estados
#[derive(Debug)]
pub struct Context {
    pub(crate) map: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn state<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(downcast_owned)
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
