use dominator::{clone, events, html, Dom};
use futures_signals::signal::SignalExt;
use std::sync::Arc;

use crate::{App, Config, GameStates, GameTheme };

pub struct InitialScreen;

impl InitialScreen {
    pub fn render(&self, app: Arc<App>) -> Dom {
        let base = "initial";
        html! {"section", {
            .class(base)
            .visible_signal(app.state().map(|s| s == GameStates::Initial))
            .children(&mut [
                      html!("h1", {
                        .class(format!("{}_title", base))
                        .text("memory")
                      }),
                    self.render_config(app),
            ])
        }}
    }

    pub fn render_config(&self, app: Arc<App>) -> Dom {
        let base = "config";
        let number_selected = app.config.signal_ref(|val| val.theme == GameTheme::Numbers);
        let icons_selected = app.config.signal_ref(|val| val.theme == GameTheme::Icons);

        let size4_selected = app.config.signal_ref(|val| val.size == 16);
        let size6_selected = app.config.signal_ref(|val| val.size == 36);

        html! {"div",{
            .class(base)
            .children(&mut [
                      html!("div", {
                          .class("row")
                          .children(&mut [
                            html!{"h3", {
                                .class(format!("{}_label", base))
                                .text("Select Theme")
                            }},
                            html!{"div", {
                                .class(format!("{}_options", base))
                                .children(&mut[
                                    html!{"button", {
                                        .class("btn")
                                        .class_signal("selected", number_selected)
                                        .class("bg_gray_100")
                                        .text("Numbers")
                                        .event(clone!(app => move |_: events::Click| {
                                            app.config.replace_with(|cfg| Config{ theme: GameTheme::Numbers, ..*cfg});
                                        }))
                                    }},
                                    html!{"button", {
                                        .class("btn")
                                        .class_signal("selected", icons_selected)
                                        .class("bg_gray_100")
                                        .text("Icons")
                                        .event(clone!(app => move |_: events::Click| {
                                            app.config.replace_with(|cfg| Config{ theme: GameTheme::Icons, ..*cfg});
                                        }))
                                    }}
                                ])
                            }},

                          ])
                      }),

                    html!("div", {
                          .class("row")
                          .children(&mut [
                            html!{"h3", {
                                .class(format!("{}_label", base))
                                .text("Number of Players")
                            }},
                            html!{"div", {
                                .class(format!("{}_options", base))
                                .children(&mut[
                                    html!{"button", {
                                        .class("btn")
                                        .class_signal("selected", app.config.signal_ref(|v| v.players == 1 ))
                                        .class("bg_gray_100")
                                        .text("1")
                                        .event(clone!(app => move |_: events::Click| {
                                            app.config.replace_with(|cfg| Config{ players: 1, ..*cfg});
                                        }))
                                    }},
                                    html!{"button", {
                                        .class("btn")
                                        .class_signal("selected", app.config.signal_ref(|v| v.players == 2 ))
                                        .class("bg_gray_100")
                                        .text("2")
                                        .event(clone!(app => move |_: events::Click| {
                                            app.config.replace_with(|cfg| Config{ players: 2, ..*cfg});
                                        }))
                                    }},
                                    html!{"button", {
                                        .class("btn")
                                        .class_signal("selected", app.config.signal_ref(|v| v.players == 3 ))
                                        .class("bg_gray_100")
                                        .text("3")
                                        .event(clone!(app => move |_: events::Click| {
                                            app.config.replace_with(|cfg| Config{ players: 3, ..*cfg});
                                        }))
                                    }},
                                    html!{"button", {
                                        .class("btn")
                                        .class_signal("selected", app.config.signal_ref(|v| v.players == 4 ))
                                        .class("bg_gray_100")
                                        .text("4")
                                        .event(clone!(app => move |_: events::Click| {
                                            app.config.replace_with(|cfg| Config{ players: 4, ..*cfg});
                                        }))
                                    }}
                                ])
                            }},

                          ])
                      }),

                        html!("div", {
                          .class("row")
                          .children(&mut [
                            html!{"h3", {
                                .class(format!("{}_label", base))
                                .text("Grid Size")
                            }},
                            html!{"div", {
                                .class(format!("{}_options", base))
                                .children(&mut[
                                    html!{"button", {
                                        .class("btn")
                                        .class_signal("selected", size4_selected)
                                        .class("bg_gray_100")
                                        .text("4x4")
                                        .event(clone!(app => move |_: events::Click| {
                                            App::change_size(app.clone(), 16);
                                        }))
                                    }},
                                    html!{"button", {
                                        .class("btn")
                                        .class_signal("selected", size6_selected)
                                        .class("bg_gray_100")
                                        .text("6x6")
                                        .event(clone!(app => move |_: events::Click| {
                                            App::change_size(app.clone(), 36);
                                        }))
                                    }}
                                ])
                            }},

                          ])
                      }),

                      html!("div", {
                          .class("row")
                          .children(&mut [
                            html!{"div", {
                                .class(format!("{}_options", base))
                                .children(&mut[
                                    html!{"button", {
                                        .class("btn")
                                        .class("big")
                                        .class("bg_orange")
                                        .text("Start Game")
                                        .event(clone!(app => move |_: events::Click| {
                                            app.state.replace_with(|_state| GameStates::Playing);
                                        }))
                                    }}
                                ])
                            }},

                          ])
                      }),

            ])
        }}
    }
}
