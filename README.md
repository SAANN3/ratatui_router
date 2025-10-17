# ratatui_router

## ratatui_router

This crate helps with automatic building page-based navigation
for [`ratatui`](https://docs.rs/ratatui).

**Features**
- derive macro [ratatui_router_derive::Routes] that automatically generates routing with pages, from your enum
- Page rendering
- Event handling
- Shared context between pages
- Convenient way to switch pages

### Quick intro
This package was inspired by dioxus with their awesome dioxus-router. So here is something similar but for ratatui.

This library is experimental and may still change or break in future versions.
Contributions, ideas, and issue reports are welcome on [GitHub](https://github.com/SAANN3/ratatui_router)!

### Example
```rust
use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::widgets::{Block, Paragraph};
use ratatui_router::ratatui_router_derive::Routes;
use ratatui_router::router::{Events, Routed, Router};

#[derive(Routes)]
pub enum MyRoutes {
    Home { counter: i64 },
    Modifier,
}

pub fn Home(ctx: &mut Router<MyRoutes>, frame: &mut Frame, counter: &mut i64) {
    let modifier = ctx.get_context::<i64>();
    if let Some(ev) = ctx.event() {
        match ev {
            Events::Event(crossterm::event::Event::Key(key)) => match key.code {
                KeyCode::Esc => ctx.exit(),
                KeyCode::Tab => ctx.change_page(MyRoutes::Modifier),
                _ => *counter += 1 * *modifier.borrow(),
            },
            _ => {}
        }
    }
    let paragraph = Paragraph::new(
        format!("Current counter = {}, press escape to exit, tab to change page, or any other button to increment to {}", counter, *modifier.borrow())
    )
        .block(Block::bordered().title("Home"));
    frame.render_widget(paragraph, frame.area());
}

pub fn Modifier(ctx: &mut Router<MyRoutes>, frame: &mut Frame) {
    let modifier = ctx.get_context::<i64>();
    if let Some(ev) = ctx.event() {
        match ev {
            Events::Event(crossterm::event::Event::Key(key)) => match key.code {
                KeyCode::Esc => ctx.exit(),
                KeyCode::Tab => {
                    ctx.go_back();
                }
                _ => *modifier.borrow_mut() += 1,
            },
            _ => {}
        }
    }
    let paragraph = Paragraph::new(
        format!("Current modifier = {}, press escape to exit, tab to change page, or any other button to increment", *modifier.borrow())
    )
        .block(Block::bordered().title("Modifier"));
    frame.render_widget(paragraph, frame.area());
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut router = MyRoutes::create_router(MyRoutes::Home { counter: 0 });
    router.create_context::<i64>(1); // You can also create context inside pages
    router.run(terminal)?;
    ratatui::restore();
    Ok(())
}

```
### Custom events
The [`Routes`](ratatui_router_derive::Routes) macro also allows you to associate **custom event types**
with pages. These can be retrieved later via [`Router::event`](crate::router::Router::event).

See [`Routed::Ev`](crate::router::Routed::Ev) for more details on how events are generated.

### Todo
- Router but with tabs instead of pages
- Some analog of use_effect with passed callback for events?
- Global callback that will be called in all pages
---

Copyright (c) [SAANN3](https://github.com/SAANN3)
This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)
