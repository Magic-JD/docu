use crate::database::data_types::ScriptletData;
use crate::database::scriptlet::match_scriptlets;
use crate::database::{scriptlet, tool, tool_to_scriptlet};
use crate::errors::error::DocuError;
use crate::errors::error::DocuError::Access;
use dirs::data_dir;
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

static CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open(database_path()).expect("Failed to open database");
    conn.execute_batch(
        "
        BEGIN;
        CREATE TABLE IF NOT EXISTS scriptlet (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL UNIQUE,
            command     TEXT NOT NULL,
            description TEXT,
            time        DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS tool (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            time DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS tool_scriptlet (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            tool_id       INTEGER NOT NULL,
            scriptlet_id  INTEGER NOT NULL,
            UNIQUE(tool_id, scriptlet_id),
            FOREIGN KEY(tool_id)      REFERENCES tool(id),
            FOREIGN KEY(scriptlet_id) REFERENCES scriptlet(id)
        );
        CREATE VIRTUAL TABLE IF NOT EXISTS scriptlet_fts
        USING fts5(
            name,
            description,
            command,
            tokenize='porter',
            content='scriptlet',
            content_rowid='id'
        );
        CREATE TRIGGER IF NOT EXISTS scriptlet_ai AFTER INSERT ON scriptlet BEGIN
          INSERT INTO scriptlet_fts(rowid, name, description, command)
            VALUES (new.id, new.name, new.description, new.command);
        END;
        CREATE TRIGGER IF NOT EXISTS scriptlet_ad AFTER DELETE ON scriptlet BEGIN
          INSERT INTO scriptlet_fts(scriptlet_fts, rowid, name, description, command)
            VALUES('delete', old.id, old.name, old.description, old.command);
        END;
        CREATE TRIGGER IF NOT EXISTS scriptlet_au AFTER UPDATE ON scriptlet BEGIN
          INSERT INTO scriptlet_fts(scriptlet_fts, rowid, name, description, command)
            VALUES('delete', old.id, old.name, old.description, old.command);
          INSERT INTO scriptlet_fts(rowid, name, description, command)
            VALUES (new.id, new.name, new.description, new.command);
        END;
        COMMIT;
        ",
    )
    .expect("Failed to initialize database");
    Mutex::new(conn)
});

fn database_path() -> PathBuf {
    let mut path = data_dir().expect("Failed to obtain data directory");
    path.push("docu");
    create_dir_all(&path).expect("Failed to create data directory");
    path.push("docu.db");
    path
}

fn get_conn() -> Result<MutexGuard<'static, Connection>, DocuError> {
    CONNECTION.lock().map_err(|e| Access(e.to_string()))
}

pub fn add_scriptlet(
    title: &str,
    tools: Vec<&str>,
    command: &str,
    description: &str,
) -> Result<(), DocuError> {
    let conn = get_conn()?;
    let scriptlet_idx = scriptlet::insert_row(title, command, description, &conn)?;
    for tool in tools {
        let tool_idx = tool::add_or_get_tool(tool, &conn)?;
        tool_to_scriptlet::link_scriptlet_to_tool(tool_idx, scriptlet_idx, &conn)?;
    }
    Ok(())
}

pub fn get_all_scriptlets() -> Result<Vec<ScriptletData>, DocuError> {
    let conn = get_conn()?;
    scriptlet::get_scriptlets(&conn)
}

pub fn get_scriptlets_for_tool(tool_name: &str) -> Result<Vec<ScriptletData>, DocuError> {
    let conn = get_conn()?;
    let tool_id =
        tool::get_tool(tool_name, &conn).expect("There are no scriptlets that use this tool");
    tool_to_scriptlet::get_from_tool_id(tool_id, &conn)
}
pub fn search_scriptlets(query: &str) -> Result<Vec<ScriptletData>, DocuError> {
    let conn = get_conn()?;
    match_scriptlets(query, &conn)
}
