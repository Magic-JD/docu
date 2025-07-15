use crate::config::load::CONFIG;
use crate::database::data_types::ScriptletData;

use crate::tui::syntax_highlight::highlight_code;
use nu_ansi_term::{Color, Style};

pub fn show_all_scriptlets_tui(scriptlets: Vec<ScriptletData>) {
    let items = convert_to_list_items(scriptlets);
    println!();
    for item in items {
        println!("{item}");
    }
}

fn convert_to_list_items(scriptlets: Vec<ScriptletData>) -> Vec<String> {
    let items: Vec<String> = scriptlets
        .into_iter()
        .flat_map(|s| {
            vec![
                Style::new()
                    .fg(Color::Rgb(
                        CONFIG.colors.scriptlet_name.unwrap_or_default().r,
                        CONFIG.colors.scriptlet_name.unwrap_or_default().g,
                        CONFIG.colors.scriptlet_name.unwrap_or_default().b,
                    ))
                    .bold()
                    .paint(format!("{} ({})", s.name, s.id))
                    .to_string(),
                Style::new()
                    .fg(Color::Rgb(
                        CONFIG.colors.scriptlet_description.unwrap_or_default().r,
                        CONFIG.colors.scriptlet_description.unwrap_or_default().g,
                        CONFIG.colors.scriptlet_description.unwrap_or_default().b,
                    ))
                    .paint(s.description)
                    .to_string(),
                highlight_code(&s.command.clone()),
                String::new(),
            ]
        })
        .collect();
    items
}
