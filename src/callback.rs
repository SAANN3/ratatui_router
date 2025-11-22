use std::collections::HashMap;

pub(crate) struct CallbackHolder<T> {
    pub functions: Option<HashMap<usize, Box<dyn FnMut(&mut T)>>>,
    i: usize,
}

impl<T> CallbackHolder<T> {
    pub fn new() -> Self {
        Self {
            functions: Some(HashMap::new()),
            i: 0,
        }
    }

    pub fn increment(&mut self) -> usize {
        self.i += 1;
        self.i
    }

    pub fn get_functions(&mut self) -> HashMap<usize, Box<dyn FnMut(&mut T)>> {
        self.functions.take().unwrap()
    }

    pub fn set_functions(&mut self, funcs: HashMap<usize, Box<dyn FnMut(&mut T)>>) {
        self.functions = Some(funcs);
    }

    pub fn add_callback<F: FnMut(&mut T) + 'static>(&mut self, callback: F) -> usize {
        let i = self.increment();
        self.functions
            .as_mut()
            .expect("Failed to insert new callback")
            .insert(i, Box::new(callback));
        i
    }

    pub fn remove_callback(&mut self, pos: usize) {
        self.functions
            .as_mut()
            .expect("Failed to insert new callback")
            .remove(&pos);
    }

    pub fn provoke_without_borrow(
        value: &mut T,
        mut functions: HashMap<usize, Box<dyn FnMut(&mut T)>>,
    ) -> HashMap<usize, Box<dyn FnMut(&mut T)>> {
        for (_, k) in &mut functions {
            k(value);
        }
        functions
    }

    #[allow(dead_code)]
    pub fn provoke(&mut self, value: &mut T) {
        for (_, k) in self.functions.as_mut().unwrap() {
            k(value);
        }
    }
}

/// Trait that handles global callback
#[allow(private_bounds)]
pub trait SelfCallbackable: SelfCallbackablePrivate {
    /// Registers global callback, returning it's id
    /// Callbacks will be called before every render
    /// # Example
    /// ```
    /// use color_eyre::Result;
    /// use crossterm::event::{self, Event};
    /// use ratatui::{DefaultTerminal, Frame};
    /// use ratatui_router::{router::*, event::*, callback::*};
    /// use ratatui_router_derive::Routes;
    /// #[derive(Routes)]
    /// pub enum MyRoutes {
    ///     Test1 { a: String, b: usize },
    /// }
    /// pub fn Test1(ctx: &mut Router<MyRoutes>, frame: &mut Frame, a: &mut String, b: &mut usize) -> () {
    ///     // do render and event handling stuff here
    /// }
    /// pub fn main() -> Result<()> {
    ///    let mut router = MyRoutes::create_router(MyRoutes::Test1{a: "Hello!".to_string(), b: 0});
    ///    // Store id so we can remove callback later if we want
    ///    let id = router.add_callback(|router| {
    ///         // You can do all stuff here, with passed router
    ///    });
    ///    router.remove_callback(id);
    ///    Ok(())
    /// }
    /// ```
    fn add_callback<F: FnMut(&mut Self) + 'static>(&mut self, callback: F) -> usize {
        self.get_callback().add_callback(callback)
    }

    /// Removes previously registered global callback with its id.
    /// For more see [SelfCallbackable::add_callback] 
    fn remove_callback(&mut self, id: usize) {
        self.get_callback().remove_callback(id);
    }
}

impl<T: SelfCallbackablePrivate> SelfCallbackable for T {}

pub(crate) trait SelfCallbackablePrivate
where
    Self: Sized,
{
    fn get_callback(&mut self) -> &mut CallbackHolder<Self>;

    fn provoke(&mut self) {
        let mut funcs = self.get_callback().get_functions();
        funcs = CallbackHolder::provoke_without_borrow(self, funcs);
        self.get_callback().set_functions(funcs);
    }
}


