use crate::database::connect::add_scriptlet;
use dialoguer::Input;
use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;
use regex::Regex;

pub fn parse_scriptlet(scriptlet_string: &str) {
    let scriptlet = arg_regex()
        .find_iter(scriptlet_string)
        .map(|m| m.as_str().to_string())
        .collect();
    let scriptlet = replace_variables(scriptlet);
    let name = get_input("Enter the name for your scriptlet");
    let tools = scriptlet_string
        .split('|')
        .map(str::trim)
        .filter_map(|s| s.split(' ').next())
        .collect::<Vec<&str>>();
    let command = scriptlet.join(" ");
    let description = get_input("Enter the description for your scriptlet");
    add_scriptlet(&name, tools, &command, &description)
        .expect("Scriptlet could not be added to database.");
}

fn get_input(description: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(description)
        .validate_with(|user_answer: &String| {
            if user_answer.trim().is_empty() {
                let red = Style::new().red();
                Err(red.apply_to("cannot be empty").to_string())
            } else {
                Ok(())
            }
        })
        .interact_text()
        .expect("Failed to read input")
}

fn replace_variables(scriptlet: Vec<String>) -> Vec<String> {
    scriptlet.into_iter().map(replace_file).collect()
}

fn replace_file(arg: String) -> String {
    if file_regex().is_match(&arg) {
        "$FILE".into()
    } else {
        arg
    }
}

fn file_regex() -> Regex {
    Regex::new(r".*\.[A-Za-z0-9]+$").expect("Regex could not be compiled.")
}

fn arg_regex() -> Regex {
    Regex::new(r#""[^"]*"|'[^']*'|\S+"#).expect("Regex could not be compiled.")
}
