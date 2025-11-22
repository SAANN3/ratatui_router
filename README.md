# ratatui_router

## ratatui_router

<p align="center">
  <a href="https://ratatui.rs"><img src="https://ratatui.rs/built-with-ratatui/badge.svg" /></a>
  <a href="https://crates.io/crates/ratatui_router"><img src="https://img.shields.io/crates/d/ratatui_router?style=flat-square&logo=rust&logoColor=orange&label=downloads&color=orange" /></a>
  <a href="https://crates.io/crates/ratatui_router/versions"><img src="https://img.shields.io/crates/v/ratatui_router?style=flat&logo=rust&logoColor=orange&label=crates.io&color=orange" /></a>
  <a href="https://docs.rs/ratatui_router"><img src="https://img.shields.io/docsrs/ratatui_router?style=flat&logo=rust&logoColor=orange&color=orange" /></a>
  <a href="https://github.com/SAANN3/ratatui_router"><img src="https://img.shields.io/badge/github-repo-blue?style=flat&logo=github&labelColor=grey" /></a>
</p>

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

List of different routers:
- [`Router`](router::Router) - basic router

Some [`examples`](https://github.com/SAANN3/ratatui_router/tree/main/examples) repos for quick start
### Example
```rust
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::Frame;
use ratatui::widgets::{Block, Paragraph};
use ratatui_router::callback::SelfCallbackable;
use ratatui_router::ratatui_router_derive::Routes;
use ratatui_router::router::{EventHook, Events, Routed, Router};
#[derive(Routes)]
pub enum MyRoutes {
    Home { counter: i64 },
    Modifier,
}

pub fn Home(ctx: &mut Router<MyRoutes>, frame: &mut Frame, counter: &mut i64) {
    let modifier = ctx.get_context::<i64>();
    ctx.use_event(|ctx, ev| match ev {
        crossterm::event::Event::Key(key) => match key.code {
            KeyCode::Tab => ctx.change_page(MyRoutes::Modifier),
            _ => *counter += 1 * *modifier.borrow(),
        },
        _ => {}
    });

    let paragraph = Paragraph::new(
        format!("Current counter = {}, press escape to exit, tab to change page, or any other button to increment to {}", counter, *modifier.borrow())
    )
        .block(Block::bordered().title("Home"));
    frame.render_widget(paragraph, frame.area());
}

pub fn Modifier(ctx: &mut Router<MyRoutes>, frame: &mut Frame) {
    let modifier = ctx.get_context::<i64>();
    match ctx.get_event() {
        Events::Event(crossterm::event::Event::Key(key)) => match key.code {
            KeyCode::Tab => {
                ctx.go_back();
            }
            _ => *modifier.borrow_mut() += 1,
        },
        _ => {}
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
    // Register global callback for exiting
    router.add_callback(|ctx| {
        ctx.use_event(|ctx, ev| match ev {
            crossterm::event::Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => ctx.exit(),
                KeyCode::Char(c)
                    if c == 'c' && key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    ctx.exit()
                }
                _ => {}
            },
            _ => {}
        });
    });
    router.create_context::<i64>(1); // you can also create context inside pages
    router.run(terminal)?;
    ratatui::restore();
    Ok(())
}

```
### Custom events
The [`Routes`](ratatui_router_derive::Routes) macro also allows you to associate **custom event types**
with pages. These can be retrieved later via [EventHook::get_event](crate::event::EventHook::get_event).

See [`Routed::Ev`](crate::router::Routed::Ev) for more details on how events are generated.
---

Copyright (c) [SAANN3](https://github.com/SAANN3)
This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

License: MIT
