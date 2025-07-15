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
    pub scriptlet_name: Color,
    #[serde(default = "default_scriptlet_description_color")]
    pub scriptlet_description: Color,
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
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

fn default_scriptlet_name_color() -> Color {
    Color {
        r: 255,
        g: 255,
        b: 0,
    }
}

fn default_scriptlet_description_color() -> Color {
    Color {
        r: 255,
        g: 255,
        b: 255,
    }
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
