use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};
/// Global state handler
pub struct Context {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Creates context for passed value, if context already exists it will be replaced with a new value.
    pub fn create_context<T: Any>(&mut self, t: T) {
        self.map
            .insert(t.type_id(), Box::new(Rc::new(RefCell::new(t))));
    }

    /// Get shared value for passed type
    /// 
    /// Panics if context hasn't created before with [Context::create_context]
    pub fn get_context<T: Any>(&self) -> Rc<RefCell<T>> {
        let type_id = TypeId::of::<T>();
        let item = self.map.get(&type_id).expect("Context not created");
        let borrowed = item
            .downcast_ref::<Rc<RefCell<T>>>()
            .expect("Type mismatch")
            .clone();
        borrowed
    }

    /// Returns true if context exists
    pub fn is_context_exists<T: Any>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }
}
#[cfg(test)]
#[test]
fn test_context() {
    let mut context = Context::new();
    assert_eq!(context.is_context_exists::<i64>(), false);
    context.create_context::<i64>(12);
    assert_eq!(context.is_context_exists::<i64>(), true);
    context.create_context::<i8>(1);
    let val = context.get_context::<i64>();
    assert_eq!(*val.borrow(), 12);
    *val.borrow_mut() += 12;
    assert_eq!(*val.borrow(), 24);
    let val_i8 = context.get_context::<i8>();
    let mut val_i8_mut = val_i8.borrow_mut();
    let val2 = context.get_context::<i64>();
    assert_eq!(*val2.borrow(), 24);
    *val_i8_mut = 12;
}
