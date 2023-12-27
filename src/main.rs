use lazy_static::lazy_static;
use std::io::{self, *};
use termion::{
    clear, color, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    style,
};

/* Note to self:
 *
 *  DO NOT USE "cursor::Goto(pos,row)" MANUALLY
 *
 *  JUST ADD "\r" at end of each line
 *  Otherwise things will be written and overwritten ... ad infinitum
 *
 *
 *
 */

#[allow(unused_macros)]
macro_rules! strNR {
    ($expr: ident) => {{
        let s = String::from_iter($expr.chars().chain("\n\r".chars()));
        s
    }};
}

#[allow(dead_code)]
enum Juche {
    One(),
    Two(),
}

//lazy_static! {
// #[derive(Debug)]
//    static ref SEPARATOR: String = String::from("==============================================");
//}

const SEP: &'static str = "=====================================";
const START_STRINGS: [&'static str; 8] = [
    "Welcome to Terminal Todo List v1.0",
    "Made by: Lucius Y. Men, Powered by: Rust",
    SEP,
    " > Use UP and DOWN to select a different item",
    " > Press F when an item is selected, to mark it as COMPLETE",
    " > Press N to start typing a new item to be added to the todo list; press ESC to finish typing",
    " > Press ESC to exit program.",
    SEP
];
const START_STRINGS_LEN: usize = START_STRINGS.len();

const ITEM_COLOR_NON_HIGHLIGHT: color::LightBlue = color::LightBlue;
const ITEM_COLOR_HIGHLIGHT: color::LightRed = color::LightRed;
const ITEM_FCOLOR_FINISHED: color::Blue = color::Blue;
const ITEM_FCOLOR_ONGOING: color::LightGreen = color::LightGreen;

/**
 *
 *
 *
 *
 * Items to be stored here
 *
 *
 */
#[derive(Debug)]
struct TodoItem {
    name: String,
    is_done: bool,
}

impl Default for TodoItem {
    fn default() -> Self {
        Self::new("")
    }
}
impl TodoItem {
    fn new(name: &str) -> Self {
        Self {
            name: strNR!(name),
            is_done: false,
        }
    }
}

const TODOS: [&'static str; 6] = [
    "Laugh twice per day",
    "Drink 5 cups of water",
    "Implement a TodoList CLI in Rust",
    "Learn React in order to trash React",
    "SQL 1. Install PostgreSQL on Linux Mint",
    "SQL 2. Learn SQL via PostgreSQL",
];

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

fn clear_line_after(stdout: &mut RawTerminal<Stdout>, line_idx: u16) -> Result<()> {
    write!(
        stdout,
        "{}{}",
        cursor::Goto(1, line_idx),
        clear::AfterCursor
    )?;
    Ok(())
}

fn render_list(
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
            //clear::All,
            //cursor::Goto(1, 2),
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
//
//
//
//
//

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    let mut todos: Vec<TodoItem> = TODOS.iter().map(|s| TodoItem::new(s)).collect();

    let start_print =
        START_STRINGS
            .iter()
            .map(|s| strNR!(s))
            .fold(String::new(), |mut acc, next| {
                acc.push_str(&next);
                acc
            });

    write!(
        stdout,
        "{}{}{}{}{}{}{}",
        cursor::Goto(1, 1),
        clear::All,
        //
        color::Fg(color::White),
        style::Bold,
        //
        start_print,
        cursor::Hide,
        color::Fg(ITEM_COLOR_NON_HIGHLIGHT),
    )?;

    stdout.flush()?;

    /* print things in list */

    if todos.is_empty() {
        todos.push(TodoItem::new("-- There are no things to be done yet..."));
    }

    render_list(&mut stdout, &todos, 0)?;
    //    for todo in todos.iter() {
    //        write!(
    //            stdout,
    //            " {}{}{}",
    //clear::All,
    //cursor::Goto(1, 2),
    //            cursor::Hide,
    //            todo.name,
    //            todo.get_bg_color()
    //        )?;
    //        stdout.flush()?;
    //    }

    // this is the selected line
    let mut selected_line_idx = 0usize;
    let mut list_changed = false;

    for c in stdin.keys() {
        match c? {
            Key::Esc => {
                writeln!(
                    stdout,
                    //                    "{}{}{}{}",
                    "{}{}",
                    clear::All,
                    cursor::Goto(1, 1),
                    //                    color::Bg(color::Reset),
                    //                    color::Fg(color::Reset)
                )?;
                break;
            }
            Key::Char(c) => {
                if c == 'd' {
                    if !todos.is_empty() && selected_line_idx <= todos.len() - 1 {
                        todos.remove(selected_line_idx);
                        list_changed = true;
                    }
                }
                /* NOTE:
                 * Entering new item
                 */
                else if c == 'n' {
                    let mut user_input = String::with_capacity(300);
                    let curr_idx = (START_STRINGS_LEN + todos.len()) as u16 + 5;
                    /* should be more than enough */

                    loop {
                        if let Some(Ok(k)) = io::stdin().keys().next() {
                            match k {
                                Key::Esc => {
                                    write!(
                                        stdout,
                                        "{}{}",
                                        cursor::Goto(1, curr_idx),
                                        clear::AfterCursor
                                    )?;
                                    break;
                                }
                                Key::Char(ch) => {
                                    user_input.push(ch);
                                }
                                Key::Backspace => {
                                    user_input.pop();
                                }
                                _ => {}
                            }

                            writeln!(
                                stdout,
                                "{}{}You're entering (press END to finish entering):{}",
                                cursor::Goto(1, curr_idx),
                                clear::AfterCursor,
                                user_input
                            )?;
                        }
                    }

                    //                    io::stdin().read_line(&mut user_input)?;
                    if !user_input.is_empty() {
                        user_input.push_str("\r");
                        todos.push(TodoItem::new(&user_input));
                        cursor::Goto(1, (START_STRINGS_LEN + 1) as u16);
                        list_changed = true;
                    }
                }
                /* NOTE:
                 * Marking item as finished / unfinished
                 */
                else if c == 'f' {
                    let fini = &mut todos
                        .get_mut(selected_line_idx)
                        .ok_or(Error::new(ErrorKind::NotFound, ""))?
                        .is_done;
                    *fini = if *fini { false } else { true };

                    list_changed = true;
                }
            }
            // Key::Ctrl(c) => println!("*{}\r", c),
            Key::Left => println!("<=\r"),
            Key::Right => println!("=>\r"),
            //
            //
            Key::Up => {
                list_changed = true;
                match selected_line_idx.checked_sub(1) {
                    Some(idx) => {
                        selected_line_idx = idx;
                    }
                    // This means we should wrap around back to the highest
                    None => {
                        selected_line_idx = todos.len() - 1;
                    }
                }
            }
            Key::Down => {
                list_changed = true;
                match selected_line_idx >= todos.len() - 1 {
                    true => {
                        selected_line_idx = 0;
                    }
                    false => {
                        selected_line_idx += 1;
                    }
                }
            }
            //
            //
            Key::Backspace => println!("x\r"),
            _ => {}
        }

        /* user changed selection OR new item; re-render list */
        if list_changed {
            clear_line_after(&mut stdout, START_STRINGS_LEN as u16 + 1)?;
            render_list(&mut stdout, &todos, selected_line_idx)?;
        }

        list_changed = false;
        stdout.flush()?;
    }
    write!(stdout, "{}", cursor::Show)?;

    Ok(())
}
