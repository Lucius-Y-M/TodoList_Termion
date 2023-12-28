use std::fs::{self, File};

use lazy_static::lazy_static;
use regex::Regex;

use crate::todoitem::TodoItem;

/**
 * FILE FORMAT:
 *
 *
 * ItemName: [String],
 * IsDone: Bool,
 * Recurring: Bool,
 *
 *
 * */

const FILE_NAME: &'static str = "todoitems.txt";

pub enum SaveLoadError {
    OpenFileFailed,
    SaveFileFailed,
    //
    WipeOpFailed,
    // ParseItemFailed
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"ItemName:\[([.]+)\], IsDone:(T|F), IsRecurring:(T|F)").unwrap();
}

pub fn read_file_into_items() -> Result<Vec<TodoItem>, SaveLoadError> {
    let file = fs::read_to_string(FILE_NAME).or(Err(SaveLoadError::OpenFileFailed))?;

    let items = file.lines()
        .into_iter()
        .filter_map(|line| {
        if let Some(cap) = RE.captures(line) {
            let (name, is_done, is_recurr) = (
                String::from(cap.get(1)?.as_str()),
                match cap.get(2)?.as_str().chars().next()? {
                    'T' => Some(true),
                    'F' => Some(false),
                    _ => {
                        eprintln!(">>> WARNING: Failed to parse line \"{}\" due to incorrect IsDone Value, allowed: T or F", line);
                        None
                    }
                }?,
                match cap.get(3)?.as_str().chars().next()? {
                    'T' => Some(true),
                    'F' => Some(false),
                    _ => {
                        eprintln!(">>> WARNING: Failed to parse line \"{}\" due to incorrect IsRecurring Value, allowed: T or F", line);
                        None
                    }
                }?,
            );

            Some(TodoItem::new_full(&name, is_done, is_recurr))
        } else {
            eprintln!(">>> WARNING: Failed to parse the following line: {}; please check if line is correctly formatted.", line);
            None
        }
    })
    .collect();

    Ok(items)
}

pub fn wipe_file() -> Result<(), SaveLoadError> {
    let file = File::open(FILE_NAME).or(Err(SaveLoadError::OpenFileFailed))?;
    file.set_len(0).or(Err(SaveLoadError::WipeOpFailed))?;
    Ok(())
}
