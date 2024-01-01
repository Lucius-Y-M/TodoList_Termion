use std::io::{self, *};
use termion::{clear, color, cursor, event::Key, input::TermRead, raw::IntoRawMode, style};
//use tsoding_cli_todolist::{
//    clear_line_after, render_list, strNR, TodoItem, ITEM_COLOR_NON_HIGHLIGHT, START_STRINGS,
//    START_STRINGS_LEN,
//};
use tsoding_cli_todolist::*;

//lazy_static! {
// #[derive(Debug)]
//    static ref SEPARATOR: String = String::from("==============================================");
//}

const TODOS: [&'static str; 6] = [
    "Laugh twice per day",
    "Drink 5 cups of water",
    "Implement a TodoList CLI in Rust",
    "Learn React in order to trash React",
    "SQL 1. Install PostgreSQL on Linux Mint",
    "SQL 2. Learn SQL via PostgreSQL",
];

//
//
//
//
//

// TODO: Handle case where zero todoitem in list (pressing UP / DOWN would panic)
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

    // this is the selected line
    let mut selected_line_idx = 0usize;
    let mut list_changed = false;

    for c in stdin.keys() {
        match c? {
            Key::Esc => {
                writeln!(
                    stdout,
                    "{}{}",
                    clear::All,
                    cursor::Goto(1, 1),
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
                                    user_input.shrink_to_fit();
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
