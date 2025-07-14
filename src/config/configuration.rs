use ratatui::style::Color;
use serde::{Deserialize, Serialize, Serializer};
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> = LazyLock::new(get_config);

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default)]
    pub colors: ColorConfig,
}

#[derive(Deserialize, Serialize)]
pub struct ColorConfig {
    #[serde(default = "default_scriptlet_name_color")]
    pub scriptlet_name: ColorWrapper,
    #[serde(default = "default_scriptlet_description_color")]
    pub scriptlet_description: ColorWrapper,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            scriptlet_name: default_scriptlet_name_color(),
            scriptlet_description: default_scriptlet_description_color(),
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(try_from = "String")]
pub struct ColorWrapper(pub Color);

impl Serialize for ColorWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&color_to_string(self.0))
    }
}

fn color_to_string(color: Color) -> String {
    match color {
        Color::Reset => "reset".to_string(),
        Color::Black => "black".to_string(),
        Color::Red => "red".to_string(),
        Color::Green => "green".to_string(),
        Color::Yellow => "yellow".to_string(),
        Color::Blue => "blue".to_string(),
        Color::Magenta => "magenta".to_string(),
        Color::Cyan => "cyan".to_string(),
        Color::Gray => "gray".to_string(),
        Color::DarkGray => "darkgray".to_string(),
        Color::LightRed => "lightred".to_string(),
        Color::LightGreen => "lightgreen".to_string(),
        Color::LightYellow => "lightyellow".to_string(),
        Color::LightBlue => "lightblue".to_string(),
        Color::LightMagenta => "lightmagenta".to_string(),
        Color::LightCyan => "lightcyan".to_string(),
        Color::White => "white".to_string(),
        Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        Color::Indexed(_) => panic!("unsupported color"),
    }
}

impl From<ColorWrapper> for Color {
    fn from(value: ColorWrapper) -> Self {
        value.0
    }
}

impl Default for ColorWrapper {
    fn default() -> Self {
        ColorWrapper(Color::Reset)
    }
}

impl TryFrom<String> for ColorWrapper {
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
                return Ok(ColorWrapper(Color::Rgb(r, g, b)));
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
                return Ok(ColorWrapper(Color::Rgb(values[0], values[1], values[2])));
            }
            return Err("Invalid RGB color format. Use rgb(r, g, b)".to_string());
        }

        match value.to_lowercase().as_str() {
            "black" => Ok(ColorWrapper(Color::Black)),
            "red" => Ok(ColorWrapper(Color::Red)),
            "green" => Ok(ColorWrapper(Color::Green)),
            "yellow" => Ok(ColorWrapper(Color::Yellow)),
            "blue" => Ok(ColorWrapper(Color::Blue)),
            "magenta" => Ok(ColorWrapper(Color::Magenta)),
            "cyan" => Ok(ColorWrapper(Color::Cyan)),
            "gray" => Ok(ColorWrapper(Color::Gray)),
            "darkgray" => Ok(ColorWrapper(Color::DarkGray)),
            "lightred" => Ok(ColorWrapper(Color::LightRed)),
            "lightgreen" => Ok(ColorWrapper(Color::LightGreen)),
            "lightyellow" => Ok(ColorWrapper(Color::LightYellow)),
            "lightblue" => Ok(ColorWrapper(Color::LightBlue)),
            "lightmagenta" => Ok(ColorWrapper(Color::LightMagenta)),
            "lightcyan" => Ok(ColorWrapper(Color::LightCyan)),
            "white" => Ok(ColorWrapper(Color::White)),
            _ => Err(format!("Invalid color: {value}")),
        }
    }
}

fn default_scriptlet_name_color() -> ColorWrapper {
    ColorWrapper(Color::Yellow)
}

fn default_scriptlet_description_color() -> ColorWrapper {
    ColorWrapper(Color::White)
}

fn get_config() -> Config {
    let config_path = dirs::config_dir().map(|p| p.join("docu").join("config.toml"));
    load_config(config_path)
}

fn load_config(config_path: Option<PathBuf>) -> Config {
    config_path
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|c| toml::from_str(&c).ok())
        .unwrap_or_default()
}
