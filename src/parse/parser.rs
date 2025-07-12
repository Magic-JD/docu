use crate::database::database::add_scriptlet;
use once_cell::unsync::Lazy;
use regex::Regex;
use std::io;
use std::io::Write;

const FILE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r".*\.[A-Za-z0-9]+$").expect("Regex could not be compiled."));
const ARG_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""[^"]*"|'[^']*'|\S+"#).expect("Regex could not be compiled."));

pub fn parse_scriptlet(scriptlet: String) {
    let scriptlet = ARG_REGEX
        .find_iter(&scriptlet)
        .map(|m| m.as_str().to_string())
        .collect();
    let scriptlet = replace_variables(scriptlet);
    let name = get_scriptlet_name(&scriptlet);
    let tool = scriptlet
        .get(0)
        .expect("Tool could not be parsed.")
        .to_string();
    let command = scriptlet.join(" ");
    let description = get_scriptlet_description();
    add_scriptlet(&name, &tool, &command, &description)
        .expect("Scriptlet could not be added to database.");
}

fn get_scriptlet_name(scriptlet: &Vec<String>) -> String {
    println!("scriptlet: {:?}", scriptlet);
    print!("Enter the name for your scriptlet: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read input");
    buffer.trim().to_string()
}

fn get_scriptlet_description() -> String {
    print!("Enter the description for your scriptlet: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read input");
    buffer.trim().to_string()
}

fn replace_variables(scriptlet: Vec<String>) -> Vec<String> {
    scriptlet.into_iter().map(replace_file).collect()
}

fn replace_file(arg: String) -> String {
    if FILE_REGEX.is_match(&arg) {
        "$FILE".into()
    } else {
        arg
    }
}
