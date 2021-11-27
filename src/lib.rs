use dominator::{clone, events, html, Dom};
use futures::future::ready;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::default::Default;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

mod components;
mod containers;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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

#[derive(Clone, Debug)]
pub struct Config {
    pub theme: GameTheme,
    pub players: u8,
    pub size: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Card {
    value: usize,
    playable: bool,
    hidden: bool,
    selected: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Player {
    score: u8,
}

#[derive(Debug)]
pub struct App {
    state: Mutable<GameStates>,
    config: Mutable<Config>,
    players: MutableVec<Arc<Player>>,
    cards: MutableVec<Arc<Card>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: GameTheme::Numbers,
            players: 1,
            size: 16,
        }
    }
}

impl Card {
    fn new(value: usize) -> Self {
        Card {
            value,
            playable: true,
            hidden: true,
            selected: false,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player { score: 0 }
    }
}

impl App {
    async fn new() -> Arc<Self> {
        let cfg = Config::default();
        let cards = MutableVec::new();
        let players = MutableVec::new();

        for i in 0..cfg.size {
            cards.lock_mut().push_cloned(Arc::new(Card::new(i)));
        }

        for _ in 0..cfg.players {
            players.lock_mut().push_cloned(Arc::new(Player::default()));
        }

        Arc::new(Self {
            state: Mutable::new(GameStates::Initial),
            config: Mutable::new(cfg),
            players,
            cards,
        })
    }

    pub fn state(&self) -> impl Signal<Item = GameStates> {
        self.state.signal()
    }

    fn render(app: Arc<Self>) -> Dom {
        let cards = render_cards(app.clone());

        html! {"main", {
            .class("app")
            .children(&mut [
                containers::initial::InitialScreen.render(app.clone()),
                cards
            ])
        }}
    }

    pub fn change_size(app: Arc<Self>, size: usize) {
        app.config.lock_mut().size = size;
        app.cards.lock_mut().clear();

        for i in 0..size {
            app.cards.lock_mut().push_cloned(Arc::new(Card::new(i)));
        }


    }

    pub fn change_players(app: Arc<Self>, size: u8) {
        app.config.lock_mut().players = size;
        app.players.lock_mut().clear();

        for _ in 0..size {
            app.players.lock_mut().push_cloned(Arc::new(Player::default()));
        }
    }
}

pub fn render_cards(app: Arc<App>) -> Dom {
    let base = "game";

    html! {"section", {
        .class(base)
            .visible_signal(app.state().map(|s| s == GameStates::Playing))
            .children(&mut [
                html!{"div", {
                    .class(format!("{}_top", base))
                    .children(&mut [
                        html!{"h1", {
                            .class(format!("{}_title", base))
                            .text("memory")
                        }},
                        html!{"div", {
                            .class(format!("{}_options", base))
                            .children(&mut [
                                html!{"button", {
                                    .class("btn")
                                    .class("bg_orange")
                                    .text("Restart")
                                                                      }},

                                html!{"button", {
                                    .class("btn")
                                    .class("bg_gray_100")
                                    .text("New Game")
                                    .event(clone!(app => move |_: events::Click| {
                                        app.state.replace_with(|_state| GameStates::Initial);
                                    }))
                                }},

                            ])
                        }}
                    ])
                }},

                html!{"div", {
                    .class(format!("{}_board", base))
                    .class_signal(
                        format!("{}_board__four",base),
                        app.config.signal_cloned().map(|c| c.size == 16))
                    .class_signal(
                        format!("{}_board__six",base),
                        app.config.signal_cloned().map(|c| c.size == 36))
                    .children_signal_vec(
                        app.cards.signal_vec_cloned().map(
                            clone!(
                                app => move |card| {
                                    html!{"div", {
                                        .class("card")
                                        .text(&format!("{}", card.value)) 
                                    }}
                                }
                            )
                        )
                    )
                }}

            ])
    }}
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let app = App::new().await;
    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
