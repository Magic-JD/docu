use dirs::config_dir;
use once_cell::unsync::Lazy;
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{fs, io};

const FILE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r".*\.[A-Za-z0-9]+$").expect("Regex could not be compiled."));

pub fn parse_scriptlet(scriptlet: Vec<String>) {
    let docu_path = docu_path();
    let scriplets = scriptlets_file(docu_path.clone());
    let tool = tool_file(
        docu_path.clone(),
        scriptlet
            .get(0)
            .expect("Tool could not be parsed.")
            .to_string(),
    );
    let scriptlet = replace_variables(scriptlet);
    let name = get_scriptlet_name(&scriptlet);
    let mut content = format!("### {}\n", name);
    content.push_str(format!("```bash\n{}\n```\n", scriptlet.join(" ")).as_str());
    scriplets
        .try_lock()
        .expect("File could not be acquired.")
        .write(content.as_bytes())
        .expect("File could not be written.");
    let link = format!(
        "[{}](../scriptlets/scriptlets.md#{})",
        name,
        name.replace(" ", "-")
    );
    tool.try_lock()
        .expect("File could not be acquired.")
        .write(link.as_bytes())
        .expect("File could not be written.");
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

fn docu_path() -> PathBuf {
    let mut path = config_dir().expect("Unable to get config dir");
    path.push("docu");
    fs::create_dir_all(path.as_path()).expect("Unable to create docu directory");
    path
}

fn scriptlets_file(mut docu: PathBuf) -> Arc<Mutex<File>> {
    docu.push("scriptlets");
    fs::create_dir_all(docu.as_path()).unwrap();
    docu.push("scriptlets.md");
    let scriptlets_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(docu.as_path())
            .expect("Failed to open scriptlets file"),
    ));
    scriptlets_file
}
fn tool_file(mut docu: PathBuf, tool_name: String) -> Arc<Mutex<File>> {
    docu.push("tools");
    fs::create_dir_all(docu.as_path()).unwrap();
    docu.push(format!("{}.md", tool_name));
    let is_new = !docu.exists();
    let mut tool_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(docu.as_path())
            .expect(format!("Failed to open tool {} file", tool_name).as_str()),
    ));
    if is_new {
        write_tool_description(&tool_file, tool_name);
    }
    tool_file
}

fn write_tool_description(mut tool_file: &Arc<Mutex<File>>, tool: String) {
    let mut content = format!("# {}\n\n", tool);
    content.push_str("## Scriptlets\n\n");
    tool_file
        .try_lock()
        .expect("File could not be acquired.")
        .write(content.as_bytes())
        .expect("File could not be written.");
}
