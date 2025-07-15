use crate::config::configuration::CONFIG;
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
                        CONFIG.colors.scriptlet_name.r,
                        CONFIG.colors.scriptlet_name.g,
                        CONFIG.colors.scriptlet_name.b,
                    ))
                    .bold()
                    .paint(format!("{} ({})", s.name, s.id))
                    .to_string(),
                Style::new()
                    .fg(Color::Rgb(
                        CONFIG.colors.scriptlet_description.r,
                        CONFIG.colors.scriptlet_description.g,
                        CONFIG.colors.scriptlet_description.b,
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
