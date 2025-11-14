
use crossterm::event::Event;

/// Enum value that will be used as events, where `T` = [Routed::Ev]
#[derive(Clone)]
pub enum Events<T> {
    /// Events that will be created from fields marked with `#[event()]`, for details, see [Routed::Ev]
    Custom(T),
    /// Crossterm event, for example key that was pressed.
    Event(Event),
    /// Event that means last render of program and exiting render loop
    Exit,
    /// Called when page was changed via [Router::change_page] or [Router::go_back]
    PageChanged,
    /// First run of a program
    Started,
}

/// Hook for accsesing current event via callback functions (use_event, use_page_changed and etc)
/// 
#[allow(private_bounds)]
pub trait EventHook<T>: EventHookMut<T> {
    /// Get reference to current event, that have caused redraw
    ///
    /// For [Events::Custom] details, see [Routed::Ev](crate::router::Routed::Ev)
    /// # Example
    /// ```
    /// use ratatui::Frame;
    /// use ratatui_router::{router::*, event::*};
    /// use ratatui_router_derive::Routes;
    /// use std::thread;
    /// #[derive(Routes)]
    /// pub enum MyRoutes {
    ///    Test1(),
    /// }
    /// pub fn Test1(ctx: &mut Router<MyRoutes>, frame: &mut Frame) -> () {
    ///     match ctx.get_event() {
    ///         Events::Custom(_) => todo!(),
    ///         Events::Event(event) => todo!(),
    ///         Events::Exit => todo!(),
    ///         Events::PageChanged => todo!(),
    ///         Events::Started => todo!()
    ///     }
    /// }
    /// ```
    fn get_event(&self) -> &Events<T>;

    /// Called if current event was [Events::Event]
    /// # Example
    /// ```
    /// use ratatui::Frame;
    /// use ratatui_router::{router::*, event::*};
    /// use ratatui_router_derive::Routes;
    /// use std::thread;
    /// #[derive(Routes)]
    /// pub enum MyRoutes {
    ///    Test1(),
    /// }
    /// pub fn Test1(ctx: &mut Router<MyRoutes>, frame: &mut Frame) -> () {
    ///     ctx.use_event(|ctx, ev| {})
    /// }
    fn use_event<F: FnMut(&mut Self, &Event)>(&mut self, mut func: F) {
        if let Some(opt) = self.get_event_mut().take() {
            if let Events::Event(ev) = &opt {
                func(self, ev);
            }
            self.set_event(opt);
        }
    }
    /// Called if current event was [Events::Exit],
    /// For example, see [EventHook::use_event]
    fn use_exit<F: FnMut(&mut Self)>(&mut self, mut func: F) {
        if let Some(opt) = self.get_event_mut() {
            if let Events::Exit = opt {
                func(self);
            }
        }
    }

    /// Called if current event was [Events::Custom],
    /// For example, see [EventHook::use_event]
    fn use_custom_event<F: FnMut(&mut Self, &T)>(&mut self, mut func: F) {
        if let Some(opt) = self.get_event_mut().take() {
            if let Events::Custom(ev) = &opt {
                func(self, ev);
            }
            self.set_event(opt);
        }
    }

    /// Called if current event was [Events::PageChanged],
    /// For example, see [EventHook::use_event]
    fn use_page_change<F: FnMut(&mut Self)>(&mut self, mut func: F) {
        if let Some(opt) = self.get_event_mut() {
            if let Events::PageChanged = opt {
                func(self);
            }
        }
    }
}
// Traid is sealed and shouldn't be accessible outside
pub(crate) trait EventHookMut<T> {
    fn get_event_mut(&mut self) -> &mut Option<Events<T>>;
    fn set_event(&mut self, event: Events<T>);
}
