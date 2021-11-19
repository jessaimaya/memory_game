use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static THEME: Lazy<HashMap<String, String>> = Lazy::new(|| {
    HashMap::from([
        ("pale".to_string(), "#fcf7f3".to_string()),
        ("white".to_string(), "#fefefe".to_string()),
        ("primary".to_string(), "#300076".to_string()),
        ("secondary".to_string(), "#fad3ce".to_string()),
        ("blue".to_string(), "#e4e6ff".to_string()),
        ("yellow".to_string(), "#fff0db".to_string()),
        ("pink".to_string(), "#fee2f1".to_string()),
    ])
});
