use crate::database::data_types::ScriptletData;
use crate::database::scriptlet::convert_to_scriptlet_data;
use crate::errors::error::DocuError;
use crate::errors::error::DocuError::DatabaseSql;
use rusqlite::Connection;
use rusqlite::{Row, params};
use std::sync::MutexGuard;

pub fn link_scriptlet_to_tool(
    tool_id: i64,
    scriptlet_id: i64,
    conn: &MutexGuard<Connection>,
) -> Result<(), DocuError> {
    insert_tool_scriptlet(tool_id, scriptlet_id, &conn)
}

fn insert_tool_scriptlet(
    tool_id: i64,
    scriptlet_id: i64,
    conn: &MutexGuard<Connection>,
) -> Result<(), DocuError> {
    conn.execute(
        "INSERT OR IGNORE INTO tool_scriptlet (tool_id, scriptlet_id) VALUES (?1, ?2)",
        params![tool_id, scriptlet_id],
    )
    .map_err(DatabaseSql)?;
    Ok(())
}

pub fn get_from_tool_id(
    tool_id: i64,
    conn: &MutexGuard<Connection>,
) -> Result<Vec<ScriptletData>, DocuError> {
    let mut stmt = conn
        .prepare(
            "SELECT s.name, s.command, s.description
             FROM scriptlet s
             JOIN tool_scriptlet ts ON s.id = ts.scriptlet_id
             WHERE ts.tool_id = ?1
             ORDER BY s.time DESC",
        )
        .map_err(DatabaseSql)?;

    let rows = stmt
        .query_map(params![tool_id], |row: &Row| convert_to_scriptlet_data(row))
        .map_err(DatabaseSql)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(DatabaseSql)?;

    Ok(rows)
}
