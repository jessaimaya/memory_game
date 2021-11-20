use wasm_bindgen::prelude::*;
use std::default::Default;
use std::sync::Arc;
use futures_signals::signal::{Signal, Mutable};
use dominator::{Dom, html};

mod components;
mod containers;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum GameStates {
    Initial,
    Playing,
    Over,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum GameTheme {
    Numbers,
    Icons,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Config {
    pub theme: GameTheme,
    pub players: u8,
    pub size: u8,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Player {
    score: u8,
}

#[derive(Clone, Debug)]
pub struct App {
    state: Mutable<GameStates>,
    config: Mutable<Config>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: GameTheme::Numbers,
            players: 1,
            size: 4,
        }
    }
}

impl App {
    async fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutable::new(GameStates::Initial),
            config: Mutable::new(Config::default()),
        })
    }

    pub fn state(&self) -> impl Signal<Item = GameStates> {
        self.state.signal()
    }

    fn render(app: Arc<Self>) -> Dom {
        let init = containers::initial::InitialScreen.render(&app);

        // Create the DOM nodes
        html!("div", {
            .children(&mut [
                      init,
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
