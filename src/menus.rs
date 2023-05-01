use std::io;

use chrono::{Datelike, Weekday};

use crate::{
    database::Database,
    lesson::{LessonListing, LessonType},
    printmenu::{self},
};

pub fn main_menu(mut db: &mut Database, current_username: String) {
    let mut option_input = String::new();
    let current_username = current_username;

    loop {
        option_input.clear();
        printmenu::print_main_menu();

        if let Err(e) = io::stdin().read_line(&mut option_input) {
            println!("Error: {e}");
        } else {
            match option_input.trim().parse::<u8>() {
                Ok(1) => run_lesson_selection_menu(&mut db, &current_username),
                Ok(2) => todo!(),
                Ok(3) => todo!(),
                Ok(4) => todo!(),
                Ok(5) => todo!(),
                Ok(6) => break,
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }
}

fn new_booking_menu(mut db: &mut Database, current_username: String) {
    let mut input_option = String::new();

    loop {
        input_option.clear();
        printmenu::print_booking_by_view_choice();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<u8>() {
                Ok(1) => todo!(),
                Ok(2) => todo!(),
                Ok(3) => break,
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }
}
