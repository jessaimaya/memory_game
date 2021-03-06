use core::cmp::Ordering;
use dominator::{clone, events, html, Dom};
use futures::future::ready;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
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
    pub players: usize,
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
    id: usize,
    score: Mutable<u32>,
    state: Mutable<PlayerState>,
    moves: Mutable<usize>,
    time: Option<Mutable<f64>>,
    points: Mutable<bool>,
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

impl Player {
    fn new(id: usize) -> Self {
        Player {
            id,
            score: Mutable::new(0u32),
            state: Mutable::new(PlayerState::Iddle),
            moves: Mutable::new(0),
            time: None,
            points: Mutable::new(false),
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

        players.lock_mut().push_cloned(Arc::new(Player::new(0)));


        Arc::new(Self {
            state: Mutable::new(GameStates::Initial),
            config: Mutable::new(cfg),
            players,
            cards,
            player_in_turn: Mutable::new(0),
        })
    }

    pub fn restart(app: Arc<Self>) {
        let cfg = app.config.lock_ref();
        let mut cards = vec![];
        let mut players = vec![];
        
        for i in 0..cfg.size {
            let ind = i % (cfg.size / 2);
            cards
                .push(Arc::new(Card::new(ind, i as u8)));
        }

        
       for i in 0..cfg.players{
            players.push(Arc::new(Player::new(i)));
        } 
 
        app.cards.lock_mut().clear();
        // let mut r = thread_rng();
        // let sh = cards.shuffle(&mut r); 
        app.cards.lock_mut().replace_cloned(cards);
        app.players.lock_mut().replace_cloned(players);
 
    }

    pub fn state(&self) -> impl Signal<Item = GameStates> {
        self.state.signal()
    }

    pub fn go_play(app: Arc<Self>) {
        app.state.replace_with(|_state| GameStates::Playing);
        app.players.lock_mut()[0].state.set(PlayerState::Playing);
    }

    pub fn add_players(app: Arc<Self>) {
        let num = app.config.lock_ref().players;
        let mut players = vec![];
        for i in 0..num {
            players.push(Arc::new(Player::new(i)));
        } 
        app.players.lock_mut().clear();
        app.players.lock_mut().replace_cloned(players);
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

    pub fn game_play(app: Arc<Self>, c: Arc<Card>) {
        let all_cards = app.cards.lock_ref();
        let selected_cards = all_cards
            .iter()
            .filter(|cc| cc.id != c.id && cc.state.get() == CardState::Selected);
        let hidden = all_cards.iter().filter(|cc| cc.state.get() == CardState::Hidden);
        let players = app.players.lock_mut();
        let player = players.iter().filter(|p| p.id == app.player_in_turn.get());

        if hidden.count() == 0 {
            web_sys::console::log_1(&format!("GameOVER").into());
        }

        if selected_cards.clone().count() > 0 {
            for curr_card in selected_cards {
                if curr_card.value == c.value {
                    let c_c = c.clone(); 
                    let cur_c = curr_card.clone();
                    spawn_local(async move {
                        TimeoutFuture::new(300).await;
                        c_c.state.set(CardState::Fine);
                        cur_c.state.set(CardState::Fine);
                    });

                    player.clone().for_each(|p| {
                        p.points.set(true);
                        p.score.set(p.score.get() + 1);
                        let sp = p.clone();
                        spawn_local(async move {
                            TimeoutFuture::new(300).await;
                            sp.points.set(false);
                        });

                    });

                    let c_fine = c.clone();
                    let ccur = curr_card.clone();
                    spawn_local(async move {
                        TimeoutFuture::new(750).await;
                        c_fine.state.set(CardState::Shown);
                        ccur.state.set(CardState::Shown);
                    });
                } else {
                    let cc = c.clone();
                    let c_card = curr_card.clone();
                    
                    let cc_card = curr_card.clone();
                    let c_cc = c.clone();

                    spawn_local(async move {
                        TimeoutFuture::new(100).await;
                        c_cc.state.set(CardState::Wrong);
                        cc_card.state.set(CardState::Wrong);
                    });

                    app.player_in_turn.set(
                        (app.player_in_turn.get() + 1 ) % app.config.lock_ref().players);

                    spawn_local(async move {
                        TimeoutFuture::new(1000).await;
                        cc.state.set(CardState::Hidden);
                        c_card.state.set(CardState::Hidden);
                    });
                }
            }
            player.for_each(|data| data.moves.set(data.moves.get() + 1));
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
                                    .event(clone!(app => move |_: events::Click| {
                                        App::restart(app.clone());
                                        app.state.replace_with(|_state| GameStates::Playing);
                                    }))

                                }},

                                html!{"button", {
                                    .class("btn")
                                    .class("bg_gray_100")
                                    .text("New Game")
                                    .event(clone!(app => move |_: events::Click| {
                                        App::restart(app.clone());
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
                            match rng.gen_bool(0.5) {
                                true => Ordering::Less,
                                false => Ordering::Greater,
                            }
                        })
                        .map(
                            clone!(
                                app => move |card| {
                                    let c = card.clone();
                                    html!{"div", {
                                        .class("cell")
                                        .class("animate__faster")
                                        .class("animate__animated")
                                        .class_signal("selected", c.state.signal().map(|s| s == CardState::Selected || s == CardState::Wrong))
                                        .class_signal("animate__flip", c.state.signal().map(|s| s == CardState::Selected ))
                                        .class_signal("wrong", c.state.signal().map(|s| s == CardState::Wrong ))
                                        .class_signal("fine", c.state.signal().map(|s| s == CardState::Fine))
                                        .class_signal("shown", c.state.signal().map(|s| s == CardState::Shown))

                                        .class_signal("animate__flip", c.state.signal().map(|s| s == CardState::Hidden))
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
                }},

                html!{"div", {
                    .class(format!("{}_players",base))                     
                    .children(&mut [
                        html!{"ul", {
                            .class("players_list")
                            .class_signal("one_p", app.config.signal_cloned().map(|cfg| cfg.players == 1))
                            .class_signal("two_p", app.config.signal_cloned().map(|cfg| cfg.players == 2))
                            .class_signal("three_p", app.config.signal_cloned().map(|cfg| cfg.players == 3))
                            .class_signal("four_p", app.config.signal_cloned().map(|cfg| cfg.players == 4))
                            .children_signal_vec(app.players.signal_vec_cloned()
                                .map(clone!(app => move |p| html!{"li", {
                                    .class_signal("in_turn", app.player_in_turn.signal_cloned().map(clone!( p => move |s| s == p.id)))
                                    .class("players_list__item")
                                    .class("animate__animated")
                                    .children(&mut[
                                        html!{"p", {
                                            .class("player-name")
                                            .text(&format!("Player{}", p.id + 1))
                                        }},
                                        html!{"p", {
                                            .class_signal("animate__bounceIn", p.points.signal_cloned().map(|s| s))
                                            .class("player-score")
                                            .class("animate__animated")
                                            .text_signal( p.score.signal().map(|s| format!("{}", s)))
                                        }}
                                    ])
                                }})))
                            .children_signal_vec(app.players.signal_vec_cloned()
                                 .map(clone!(app => move |p| html!{"p",{
                                    .children(&mut[
                                        html!{"span",{ 
                                            .visible_signal(app.player_in_turn.signal_cloned().map(clone!(p => move |s| s == p.id)))
                                            .text("Current turn")

                                        }}
                                     ])
                                 }})))
                        }}
                    ])
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
