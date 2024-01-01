use std::io::{Result, Stdout, Write};

use termion::{clear, color, cursor, raw::RawTerminal};

use crate::todoitem::TodoItem;

pub const SEP: &'static str = "=====================================";
pub const START_STRINGS: [&'static str; 9] = [
    "Welcome to Terminal Todo List v1.0",
    "Made by: Lucius Y. Men, Powered by: Rust",
    SEP,
    " > Use UP and DOWN to select a different item",
    " > Press F when an item is selected, to mark it as COMPLETE",
    " > Press D when an item is selected, to DELETE said item",
    " > Press N to start typing a new item to be added to the todo list; press ESC to finish typing",
    " > Press ESC to exit program.",
    SEP
];
pub const START_STRINGS_LEN: usize = START_STRINGS.len();

pub const ITEM_COLOR_NON_HIGHLIGHT: color::LightBlue = color::LightBlue;
pub const ITEM_COLOR_HIGHLIGHT: color::LightRed = color::LightRed;
pub const ITEM_FCOLOR_FINISHED: color::Blue = color::Blue;
pub const ITEM_FCOLOR_ONGOING: color::LightGreen = color::LightGreen;

enum TermColorBg {
    Highlight,
    Normal,
}
impl std::fmt::Display for TermColorBg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TermColorBg::Highlight => write!(f, "{}", color::Bg(ITEM_COLOR_HIGHLIGHT)),
            //            TermColorBg::Normal => write!(f, "{}", color::Bg(ITEM_COLOR_NON_HIGHLIGHT)),
            TermColorBg::Normal => write!(f, "{}", color::Bg(color::Reset)),
        }
    }
}

enum TermColorFg {
    Ongoing,
    Finished,
}
impl std::fmt::Display for TermColorFg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TermColorFg::Ongoing => write!(f, "{}", color::Fg(ITEM_FCOLOR_ONGOING)),
            TermColorFg::Finished => write!(f, "{}", color::Fg(ITEM_FCOLOR_FINISHED)), // do not write anything
        }
    }
}

pub fn clear_line_after(stdout: &mut RawTerminal<Stdout>, line_idx: u16) -> Result<()> {
    write!(
        stdout,
        "{}{}",
        cursor::Goto(1, line_idx),
        clear::AfterCursor
    )?;
    Ok(())
}

pub fn render_list(
    stdout: &mut RawTerminal<Stdout>,
    items: &Vec<TodoItem>,
    selected_line_idx: usize,
) -> Result<()> {
    write!(stdout, "{}", clear::AfterCursor)?;

    for (idx, todo) in items.iter().enumerate() {
        let fgcolor = if todo.is_done {
            TermColorFg::Finished
        } else {
            TermColorFg::Ongoing
        };

        let bgcolor = if idx == selected_line_idx {
            TermColorBg::Highlight
        } else {
            TermColorBg::Normal
        };

        writeln!(
            stdout,
            "{} {}{}{}{}{}{}\r",
            cursor::Goto(1, START_STRINGS_LEN as u16 + idx as u16 + 1),
            fgcolor,
            bgcolor,
            cursor::Hide,
            todo.name,
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        )?;
    }

    Ok(())
}
