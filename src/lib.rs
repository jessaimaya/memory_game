use core::cmp::Ordering;
use dominator::{clone, events, html, Dom};
use futures::future::ready;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use rand::{thread_rng, Rng};
use std::default::Default;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;

mod components;
mod containers;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum CardState {
    Hidden,
    Shown,
    Wrong,
    Fine,
    Selected,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Iddle,
    Playing,
}

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

#[derive(Debug)]
pub struct Card {
    id: u8,
    value: usize,
    state: Mutable<CardState>,
}

#[derive(Clone, Debug)]
pub struct Player {
    id: u8,
    score: Mutable<u32>,
    state: Mutable<PlayerState>,
}

#[derive(Debug)]
pub struct App {
    state: Mutable<GameStates>,
    config: Mutable<Config>,
    players: MutableVec<Arc<Player>>,
    cards: MutableVec<Arc<Card>>,
    player_in_turn: Mutable<usize>,
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
    fn new(value: usize, id: u8) -> Self {
        Card {
            id,
            value,
            state: Mutable::new(CardState::Hidden),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            id: 0,
            score: Mutable::new(0u32),
            state: Mutable::new(PlayerState::Iddle),
        }
    }
}

impl App {
    async fn new() -> Arc<Self> {
        let cfg = Config::default();
        let cards = MutableVec::new();
        let players = MutableVec::new();

        for i in 0..cfg.size {
            let ind = i % (cfg.size / 2);
            cards
                .lock_mut()
                .push_cloned(Arc::new(Card::new(ind, i as u8)));
        }

        for _ in 0..cfg.players {
            players.lock_mut().push_cloned(Arc::new(Player::default()));
        }

        Arc::new(Self {
            state: Mutable::new(GameStates::Initial),
            config: Mutable::new(cfg),
            players,
            cards,
            player_in_turn: Mutable::new(0),
        })
    }

    pub fn state(&self) -> impl Signal<Item = GameStates> {
        self.state.signal()
    }

    pub fn go_play(app: Arc<Self>) {
        app.state.replace_with(|_state| GameStates::Playing);
        app.players.lock_mut()[0].state.set(PlayerState::Playing);
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
            let ind = i % (size / 2);
            app.cards
                .lock_mut()
                .push_cloned(Arc::new(Card::new(ind, i as u8)));
        }
    }

    pub fn change_players(app: Arc<Self>, size: u8) {
        app.config.lock_mut().players = size;
        app.players.lock_mut().clear();

        for _ in 0..size {
            app.players
                .lock_mut()
                .push_cloned(Arc::new(Player::default()));
        }
    }

    pub fn game_play(app: Arc<Self>, c: Arc<Card>) {
        // let ccard = c.clone();

        let all_cards = app.cards.lock_ref();
        let selected_cards = all_cards
            .iter()
            .filter(|cc| cc.id != c.id && cc.state.get() == CardState::Selected);

        if selected_cards.clone().count() > 0 {
            for curr_card in selected_cards {
                if curr_card.value == c.value {

                    c.state.set(CardState::Fine);
                    curr_card.state.set(CardState::Fine);

                    let c_fine = c.clone();
                    let ccur = curr_card.clone();
                    spawn_local(async move {
                        TimeoutFuture::new(500).await;
                        App::set_card_shown(c_fine.clone());
                        App::set_card_shown(ccur.clone());
                    });
                } else {
                    let cc = c.clone();
                    let c_card = curr_card.clone();

                    cc.state.set(CardState::Wrong);
                    c_card.state.set(CardState::Wrong);

                    spawn_local(async move {
                        TimeoutFuture::new(500).await;
                        App::set_card_hidden(cc.clone());
                        App::set_card_hidden(c_card.clone());
                    });
                }
            }
        }
    }

    pub fn card_selection(_app: Arc<Self>, card: Arc<Card>) {
        if card.state.get() == CardState::Hidden {
            card.state.set(CardState::Selected);
        }
    }

    pub fn set_card_shown(card: Arc<Card>) {
        card.state.set(CardState::Shown);
    }

    pub fn set_card_hidden(card: Arc<Card>) { card.state.set(CardState::Hidden); } }

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
                        app.cards.signal_vec_cloned()
                        .sort_by_cloned(|_,_| {
                            let mut rng = thread_rng();
                            match rng.gen_bool(1.0 / 3.0) {
                                true => Ordering::Less,
                                false => Ordering::Greater,
                            }
                        })
                        .map(
                            clone!(
                                app => move |card| {
                                    let c = card.clone();
                                    // TODO:  implement timeout for auto-resetting selection
                                    // gloo_timers -> enable / disable selection
                                    html!{"div", {
                                        .class("cell")
                                        .class_signal("selected", c.state.signal().map(|s| s == CardState::Selected || s == CardState::Wrong))
                                        .class_signal("wrong", c.state.signal().map(|s| s == CardState::Wrong ))
                                        .class_signal("fine", c.state.signal().map(|s| s == CardState::Fine))
                                        .class_signal("shown", c.state.signal().map(|s| s == CardState::Shown))
                                        .future(
                                            c.state.signal_cloned().for_each(clone!(c, app => move |change| {
                                                if change == CardState::Selected {
                                                    App::game_play(app.clone(), c.clone());
                                                }
                                               ready(())
                                            }))
                                        )
                                        .event(clone!(app => move |_:events::Click| {
                                            if c.state.get()  != CardState::Shown {
                                                App::card_selection(app.clone(), c.clone());
                                            }
                                        }))
                                        .children(&mut [
                                            html!{"div", {
                                                .class("card")
                                                    .children(&mut[
                                                        html!{"span", {
                                                            .class("card_value")
                                                            .text(&format!("{}", card.value))

                                                        }}
                                                    ])
                                            }}
                                        ])
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
