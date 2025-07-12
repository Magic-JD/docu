use crate::database::data_types::ToolData;
use crate::errors::error::DocuError;
use crate::errors::error::DocuError::DatabaseSql;
use rusqlite::{Connection, Error, Row};
use std::sync::MutexGuard;

pub fn add_or_get_tool(name: &str, conn: &MutexGuard<Connection>) -> Result<i64, DocuError> {
    if let Ok(existing_id) = get_tool(name, &conn) {
        return Ok(existing_id);
    }
    insert_row(name, &conn)
}

pub(crate) fn get_tool(name: &str, conn: &MutexGuard<Connection>) -> Result<i64, DocuError> {
    let id = conn
        .query_row("SELECT id FROM tool WHERE name = ?", [name], |row| {
            row.get(0)
        })
        .map_err(DatabaseSql)?;
    Ok(id)
}

fn insert_row(name: &str, conn: &MutexGuard<Connection>) -> Result<i64, DocuError> {
    conn.execute("INSERT INTO tool (name) VALUES (?)", [name])
        .map_err(DatabaseSql)?;
    Ok(conn.last_insert_rowid())
}

pub fn get_tools(conn: &MutexGuard<Connection>) -> Result<Vec<ToolData>, DocuError> {
    let mut stmt = conn.prepare("SELECT name FROM tool")?;
    let scriptlets: Vec<ToolData> = stmt
        .query_map([], convert_to_tool_data)?
        .collect::<Result<_, _>>()
        .map_err(DatabaseSql)?;
    Ok(scriptlets)
}

pub(crate) fn convert_to_tool_data(row: &Row) -> Result<ToolData, Error> {
    Ok(ToolData { name: row.get(0)? })
}
