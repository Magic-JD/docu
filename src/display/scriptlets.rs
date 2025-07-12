use crate::database::data_types::ScriptletData;
use crate::database::database::{get_all_scriptlets, get_scriptlets_for_tool};

pub fn show_all_scriptlets() {
    let scriplets = get_all_scriptlets().expect("can't get scriptlets");
    scriplets.into_iter().for_each(display_scriptlet)
}

fn display_scriptlet(scriptlet: ScriptletData) {
    println!("Name: {}", scriptlet.name);
    println!("Description: {}", scriptlet.description);
    println!("{}", scriptlet.command);
    println!();
}

pub fn show_all_scriptlets_for_tool(tool_name: &str) {
    let scriplets = get_scriptlets_for_tool(tool_name).expect("can't get scriptlets");
    if scriplets.is_empty() {
        println!("No scriptlets found for {}", tool_name);
        return;
    }
    scriplets.into_iter().for_each(display_scriptlet)
}
