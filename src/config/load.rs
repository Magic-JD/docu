use super::theme::themes;
use crate::config::DEFAULT_CONFIG;
use serde::{Deserialize, Serialize, Serializer};
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> = LazyLock::new(get_config);

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub colors: ColorConfig,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct ColorConfig {
    pub theme: Option<String>,
    pub scriptlet_name: Option<Color>,
    pub scriptlet_description: Option<Color>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(try_from = "String")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b))
    }
}

impl TryFrom<String> for Color {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with('#') {
            let hex = value.trim_start_matches('#');
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|e| format!("Invalid hex value for red: {e}"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|e| format!("Invalid hex value for green: {e}"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|e| format!("Invalid hex value for blue: {e}"))?;
                return Ok(Color { r, g, b });
            }
            return Err("Invalid hex color format. Use #RRGGBB".to_string());
        }

        if value.starts_with("rgb(") && value.ends_with(')') {
            let values = value
                .trim_start_matches("rgb(")
                .trim_end_matches(')')
                .split(',')
                .map(|s| s.trim().parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|e| format!("Invalid RGB value: {e}"))?;

            if values.len() == 3 {
                return Ok(Color {
                    r: values[0],
                    g: values[1],
                    b: values[2],
                });
            }
            return Err("Invalid RGB color format. Use rgb(r, g, b)".to_string());
        }

        match value.to_lowercase().as_str() {
            "black" => Ok(Color { r: 0, g: 0, b: 0 }),
            "red" => Ok(Color { r: 255, g: 0, b: 0 }),
            "green" => Ok(Color { r: 0, g: 255, b: 0 }),
            "yellow" => Ok(Color {
                r: 255,
                g: 255,
                b: 0,
            }),
            "blue" => Ok(Color { r: 0, g: 0, b: 255 }),
            "magenta" => Ok(Color {
                r: 255,
                g: 0,
                b: 255,
            }),
            "cyan" => Ok(Color {
                r: 0,
                g: 255,
                b: 255,
            }),
            "white" => Ok(Color {
                r: 255,
                g: 255,
                b: 255,
            }),
            _ => Err(format!("Invalid color: {value}")),
        }
    }
}

fn get_config() -> Config {
    let mut config: Config = toml::from_str(DEFAULT_CONFIG).expect("Couldn't parse default config");

    let user_config_path = dirs::config_dir().map(|p| p.join("docu").join("config.toml"));
    let user_config: Option<Config> = load_config(user_config_path);

    if let Some(user_config) = user_config {
        let theme = user_config.colors.theme.or(config.colors.theme);
        let scriptlet_name = user_config
            .colors
            .scriptlet_name
            .or(config.colors.scriptlet_name);
        let scriptlet_description = user_config
            .colors
            .scriptlet_description
            .or(config.colors.scriptlet_description);

        config.colors.theme = theme;
        config.colors.scriptlet_name = scriptlet_name;
        config.colors.scriptlet_description = scriptlet_description;
    }

    let theme_name = config.colors.theme.clone().unwrap_or("default".to_string());
    let themes = themes();
    let theme = themes
        .get(&theme_name)
        .unwrap_or_else(|| themes.get("default").unwrap());

    config.colors.scriptlet_name = config.colors.scriptlet_name.or(Some(theme.scriptlet_name));
    config.colors.scriptlet_description = config
        .colors
        .scriptlet_description
        .or(Some(theme.scriptlet_description));

    config
}

fn load_config(config_path: Option<PathBuf>) -> Option<Config> {
    config_path
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|c| toml::from_str(&c).ok())
}
