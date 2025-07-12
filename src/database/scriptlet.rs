use crate::database::data_types::ScriptletData;
use crate::errors::error::DocuError;
use crate::errors::error::DocuError::DatabaseSql;
use rusqlite::{Connection, Error, Row};
use std::sync::MutexGuard;

pub(crate) fn insert_row(
    title: &str,
    command: &str,
    description: &str,
    conn: &MutexGuard<Connection>,
) -> Result<i64, DocuError> {
    conn.execute(
        "INSERT INTO scriptlet (name, command, description) VALUES (?, ?, ?)",
        [title, command, description],
    )
    .map_err(DatabaseSql)?;
    Ok(conn.last_insert_rowid())
}

pub fn get_scriptlets(conn: &MutexGuard<Connection>) -> Result<Vec<ScriptletData>, DocuError> {
    let mut stmt =
        conn.prepare("SELECT name, command, description FROM scriptlet ORDER BY time DESC")?;
    let scriptlets: Vec<ScriptletData> = stmt
        .query_map([], convert_to_scriptlet_data)?
        .collect::<Result<_, _>>()
        .map_err(DatabaseSql)?;
    Ok(scriptlets)
}

pub fn match_scriptlets(
    query: &str,
    conn: &MutexGuard<Connection>,
) -> Result<Vec<ScriptletData>, DocuError> {
    let query = query
        .split_whitespace()
        .map(|tok| tok.to_lowercase())
        .map(|tok| format!("{}*", tok))
        .collect::<Vec<_>>()
        .join(" OR ");
    let sql = "
        SELECT s.name, s.command, s.description
        FROM   scriptlet_fts
        JOIN   scriptlet     AS s ON s.id = scriptlet_fts.rowid
        WHERE  scriptlet_fts MATCH ?1
          AND  bm25(scriptlet_fts)  < ?2
        ORDER  BY bm25(scriptlet_fts);
    ";
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt
        .query_map([query, 10f32.to_string()], convert_to_scriptlet_data)?
        .collect::<Result<_, _>>()?;
    Ok(rows)
}

pub(crate) fn convert_to_scriptlet_data(row: &Row) -> Result<ScriptletData, Error> {
    Ok(ScriptletData {
        name: row.get(0)?,
        command: row.get(1)?,
        description: row.get(2)?,
    })
}
