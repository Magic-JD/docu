use crate::database::data_types::ScriptletData;

use crate::tui::syntax_highlight::highlight_code;
use crossterm::terminal::{ScrollUp, size};
use crossterm::{cursor::position, execute};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, List, ListItem},
};
use std::io;

pub fn show_all_scriptlets_tui(scriptlets: Vec<ScriptletData>) -> io::Result<()> {
    let items: Vec<ListItem> = scriptlets
        .into_iter()
        .flat_map(|s| {
            vec![
                ListItem::new(Span::styled(
                    s.name,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
                ListItem::new(Span::styled(
                    s.description,
                    Style::default().fg(Color::White),
                )),
                ListItem::new(highlight_code(s.command.clone())),
                ListItem::new(Span::raw("")),
            ]
            .into_iter()
            .collect::<Vec<_>>()
        })
        .collect();

    let tui_height = items.len() as u16 + 2;
    let (_, mut start_row) = position()?;
    let (_, rows) = size()?;
    let avail = rows.saturating_sub(start_row);
    if tui_height > avail {
        let to_scroll = tui_height - avail;
        execute!(io::stdout(), ScrollUp(to_scroll))?; // :contentReference[oaicite:0]{index=0}
        start_row = start_row - to_scroll;
    }
    // 3) Set up a normal (not-alt-screen) ratatui terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    // 4) Render only the rectangle starting at start_row
    term.draw(|f| {
        let size = f.area();
        // clamp height to total_rows
        let height = tui_height.min(size.height.saturating_sub(start_row));
        let area = Rect {
            x: 0,
            y: start_row + 1,
            width: size.width,
            height,
        };
        let list = List::new(items.clone()).block(Block::default());
        f.render_widget(list, area);
    })?;

    // 5) leave the cursor right after the last line
    execute!(
        term.backend_mut(),
        crossterm::cursor::MoveTo(0, start_row + tui_height)
    )?;
    Ok(())
}
