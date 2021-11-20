/*
use std::process::Output;

use futures_signals::signal::MutableSignalRef;
use dominator::{Dom, html};

use crate::{App, GameTheme};

pub struct Button;

impl Button {
    pub fn render(text: &str, color: &str, selected: MutableSignalRef<bool, FnMut() -> bool >) -> Dom{
        html!{"button", {
            .class("btn")
            .class_signal("selected", selected)
            .class(color)
            .text(text)
        }}
    }
}
*/
