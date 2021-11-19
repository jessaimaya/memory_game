use log::info;
use wasm_bindgen::prelude::*;
use std::sync::Arc;
use once_cell::sync::Lazy;
use futures_signals::signal::{Signal, Mutable, SignalExt};
use dominator::{Dom, class, html, clone, events, routing};
use web_sys::{Url, console};

mod theme;
mod components;

#[derive(Debug)]
pub struct App {
    counter: Mutable<i32>,
}

impl App {
    async fn new() -> Arc<Self> {
        Arc::new(Self {
            counter: Mutable::new(0),
        })
    }

    pub fn render_content(app: Arc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("div", {
                    .text("Dominator!")
                })
            ])
        })
    }

    fn render(state: Arc<Self>) -> Dom {

        // Create the DOM nodes
        html!("div", {
            .children(&mut [
                Self::render_content(state.clone()),
            ])
        })
    }
}


#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    let app = App::new().await;
    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
