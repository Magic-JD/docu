use crate::config::load::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
pub struct Theme {
    pub scriptlet_name: Color,
    pub scriptlet_description: Color,
}

pub fn themes() -> HashMap<String, Theme> {
    let mut themes = HashMap::new();
    themes.insert(
        "default".to_string(),
        Theme {
            scriptlet_name: Color {
                r: 200,
                g: 200,
                b: 0,
            },
            scriptlet_description: Color {
                r: 200,
                g: 200,
                b: 200,
            },
        },
    );
    themes.insert(
        "ocean".to_string(),
        Theme {
            scriptlet_name: Color {
                r: 0,
                g: 100,
                b: 200,
            },
            scriptlet_description: Color {
                r: 140,
                g: 180,
                b: 200,
            },
        },
    );
    themes.insert(
        "forest".to_string(),
        Theme {
            scriptlet_name: Color {
                r: 30,
                g: 120,
                b: 30,
            },
            scriptlet_description: Color {
                r: 120,
                g: 200,
                b: 120,
            },
        },
    );
    themes.insert(
        "sunset".to_string(),
        Theme {
            scriptlet_name: Color {
                r: 220,
                g: 140,
                b: 0,
            },
            scriptlet_description: Color {
                r: 220,
                g: 180,
                b: 0,
            },
        },
    );
    themes
}
