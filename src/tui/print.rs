use crate::config::configuration::CONFIG;
use crate::database::data_types::ScriptletData;

use crate::errors::error::DocuError;
use crate::tui::syntax_highlight::highlight_code;
use crossterm::terminal::{ScrollUp, size};
use crossterm::{cursor::position, execute};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, List, ListItem},
};
use std::io;

pub fn show_all_scriptlets_tui(scriptlets: Vec<ScriptletData>) -> Result<(), DocuError> {
    let items = convert_to_list_items(scriptlets);

    let (tui_height, start_row) = clear_to_start(&items)?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;
    term.draw(|f| {
        let size = f.area();
        let height = tui_height.min(size.height.saturating_sub(start_row));
        let area = Rect {
            x: 0,
            y: start_row + 1,
            width: size.width,
            height,
        };
        let list = List::new(items).block(Block::default());
        f.render_widget(list, area);
    })?;
    execute!(
        term.backend_mut(),
        crossterm::cursor::MoveTo(0, start_row + tui_height)
    )?;
    Ok(())
}

fn clear_to_start(items: &[ListItem]) -> Result<(u16, u16), DocuError> {
    let tui_height = u16::try_from(items.len()).expect("Item length too big") + 2;
    let (_, mut start_row) = position()?;
    let (_, rows) = size()?;
    let avail = rows.saturating_sub(start_row);
    if tui_height > avail {
        let to_scroll = tui_height - avail;
        execute!(io::stdout(), ScrollUp(to_scroll))?;
        start_row -= to_scroll;
    }
    Ok((tui_height, start_row))
}

fn convert_to_list_items(scriptlets: Vec<ScriptletData>) -> Vec<ListItem<'static>> {
    let items: Vec<ListItem> = scriptlets
        .into_iter()
        .flat_map(|s| {
            vec![
                ListItem::new(Span::styled(
                    format!("{} ({})", s.name, s.id),
                    Style::default()
                        .fg(CONFIG.colors.scriptlet_name.into())
                        .add_modifier(Modifier::BOLD),
                )),
                ListItem::new(Span::styled(
                    s.description,
                    Style::default().fg(CONFIG.colors.scriptlet_description.into()),
                )),
                ListItem::new(highlight_code(&s.command.clone())),
                ListItem::new(Span::raw("")),
            ]
            .into_iter()
            .collect::<Vec<_>>()
        })
        .collect();
    items
}
