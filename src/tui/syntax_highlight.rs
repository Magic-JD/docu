use nu_ansi_term::{Color, Style};
use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

pub fn highlight_code(text: &str) -> String {
    let syntax = SYNTAX_SET
        .find_syntax_by_token("bash")
        .expect("could not find syntax");
    let mut highlighter = HighlightLines::new(syntax, get_default_theme());
    highlight_line(&SYNTAX_SET, &mut highlighter, text)
}

fn highlight_line(syntax_set: &SyntaxSet, highlighter: &mut HighlightLines, line: &str) -> String {
    let highlighted_string = highlighter
        .highlight_line(line, syntax_set)
        .expect("Line could not be highlighted.");
    let styled_spans = highlighted_string
        .into_iter()
        .map(|(style, content)| convert_syntect_style(&style).paint(content).to_string())
        .collect::<Vec<String>>();
    styled_spans.join("")
}

fn get_default_theme() -> &'static Theme {
    THEME_SET
        .themes
        .get("base16-ocean.dark")
        .expect("base16-ocean.dark is not found")
}

fn convert_syntect_style(syntect_style: &syntect::highlighting::Style) -> Style {
    from_syntect_color(syntect_style.foreground)
}

fn from_syntect_color(syntext_color: syntect::highlighting::Color) -> Style {
    Style::new().fg(Color::Rgb(
        syntext_color.r,
        syntext_color.g,
        syntext_color.b,
    ))
}
