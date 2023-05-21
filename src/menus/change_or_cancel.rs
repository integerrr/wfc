use std::io;

use crate::{database::Database, lesson::LessonListing, menus::booking};

pub fn main_menu(db: &mut Database) {
    let mut input_option = String::new();

    loop {
        input_option.clear();
        print_main_menu();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<i32>() {
                Ok(1) => {
                    change_booking_menu(db);
                    break;
                }
                Ok(2) => {
                    cancel_booking_menu(db);
                    break;
                }
                Ok(3) => break,
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }
}

fn change_booking_menu(db: &mut Database) {
    let booked_lessons = db.get_and_display_booked_lessons();

    if booked_lessons.is_empty() {
        return;
    }
    let old_lesson = select_lesson(booked_lessons);

    if booking::main_menu(db).is_some() {
        db.cancel_lesson(old_lesson);
    }
}

fn cancel_booking_menu(db: &mut Database) {
    let booked_lessons = db.get_and_display_booked_lessons();

    if booked_lessons.is_empty() {
        return;
    }

    let selected_lesson = select_lesson(booked_lessons);
    db.cancel_lesson(selected_lesson);
}

fn select_lesson(lessons: Vec<LessonListing>) -> LessonListing {
    let mut input_option = String::new();
    let selected_lesson: LessonListing;

    loop {
        input_option.clear();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else if let Ok(choice) = input_option.trim().parse::<usize>() {
            if let Some(lesson) = lessons.get(choice - 1) {
                selected_lesson = lesson.clone();
                break selected_lesson;
            } else {
                println!("invalid");
            }
        } else {
            println!("wrong no help");
        }
    }
}

fn print_main_menu() {
    println!();
    println!("*******************************************");
    println!();
    println!("1. Change a booking");
    println!("2. Cancel a booking");
    println!("3. Go back");
}
