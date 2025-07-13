use crate::database::connect::{get_all_scriptlets, get_scriptlets_for_tool, search_scriptlets};
use crate::tui::print::show_all_scriptlets_tui;

pub fn show_all_scriptlets() {
    let scriplets = get_all_scriptlets().expect("can't get scriptlets");
    show_all_scriptlets_tui(scriplets).expect("Tui failed to display");
}

pub fn show_all_scriptlets_for_tool(tool_name: &str) {
    let scriplets = get_scriptlets_for_tool(tool_name).expect("can't get scriptlets");
    if scriplets.is_empty() {
        println!("No scriptlets found for {}", tool_name);
        return;
    }
    show_all_scriptlets_tui(scriplets).expect("Tui failed to display");
}

pub fn show_searched_scriptlets(search: Vec<String>) {
    let scriplets = search_scriptlets(&search.join(" ")).expect("can't get scriptlets");
    show_all_scriptlets_tui(scriplets).expect("Tui failed to display");
}
