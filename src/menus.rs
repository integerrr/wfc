use std::io;

use chrono::{Datelike, Weekday};

use crate::{
    database::Database,
    lesson::{LessonListing, LessonType},
    printmenu::{self},
};

pub fn main_menu(db: &mut Database, current_username: String) {
    let mut option_input = String::new();
    let current_username = current_username;
    dbg!(&db);

    loop {
        option_input.clear();
        printmenu::print_main_menu();

        if let Err(e) = io::stdin().read_line(&mut option_input) {
            println!("Error: {e}");
        } else {
            match option_input.trim().parse::<i32>() {
                Ok(1) => new_booking_menu(db, current_username.clone()),
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

fn new_booking_menu(db: &mut Database, current_username: String) {
    let mut input_option = String::new();

    loop {
        input_option.clear();
        printmenu::print_booking_by_view_choice();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<i32>() {
                Ok(1) => book_lesson_by_weekday_menu(db, current_username.clone()),
                Ok(2) => todo!(),
                Ok(3) => break,
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }
}

fn book_lesson_by_weekday_menu(db: &mut Database, current_username: String) {
    let mut input_option = String::new();
    let mut filtered_lessons: Vec<&LessonListing> = Vec::new();
    let chosen_lesson: LessonListing;

    loop {
        input_option.clear();
        filtered_lessons.clear();
        printmenu::print_booking_weekday_choice();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<i32>() {
                Ok(1) => {
                    filtered_lessons = get_and_display_available_lessons_by_weekday(
                        db,
                        Weekday::Sat,
                        current_username.clone(),
                    );
                    break;
                }
                Ok(2) => {
                    filtered_lessons = get_and_display_available_lessons_by_weekday(
                        db,
                        Weekday::Sun,
                        current_username.clone(),
                    );
                    break;
                }
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }

    loop {
        input_option.clear();
        println!();
        println!("Please choose the lesson you want:");

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else if let Ok(choice) = input_option.trim().parse::<usize>() {
            if let Some(&lesson) = filtered_lessons.get(choice - 1) {
                chosen_lesson = lesson.clone();
                break;
            } else {
                println!("invalid option");
            }
        } else {
            panic!("couldn't parse the option input?");
        }
    }

    db.book_lesson(chosen_lesson.clone(), current_username);

    println!();
    println!(
        "You have successfully booked a {} lesson on {} for £{}!",
        &chosen_lesson.lesson_type,
        &chosen_lesson.date.format("%d/%m/%Y %H:%M"),
        &chosen_lesson.get_price()
    );
}

fn get_and_display_available_lessons_by_weekday(
    db: &Database,
    wd: Weekday,
    current_username: String,
) -> Vec<&LessonListing> {
    let filtered_lessons: Vec<_> = db
        .lessons
        .iter()
        .filter(|lesson| {
            lesson.date.weekday() == wd
                && !(lesson
                    .students_enrolled
                    .iter()
                    .any(|user| user.username == current_username))
                && lesson.get_vacancy() > 0
        })
        .collect();

    println!(); // beautifying things
    println!("*******************************************");
    println!("Available lessons:");
    println!();

    filtered_lessons
        .iter()
        .enumerate()
        .for_each(|(index, lesson)| {
            println!(
                "{}. {} ({})",
                index + 1,
                lesson.date.format("%d/%m/%Y %H:%M"),
                &wd
            );
            println!("    {}, £{}", lesson.lesson_type, lesson.get_price());
        });

    filtered_lessons
}

fn book_lesson_by_type_menu(mut db: &mut Database, current_username: String) {
    let mut input_option = String::new();

    loop {
        input_option.clear();
        printmenu::print_booking_type_choice();
    }
}

fn get_and_display_available_lessons_by_type(db: &Database, lesson_type: LessonType) {}

fn run_available_lessons_menu() {
    let mut input_option = String::new();

    loop {
        input_option.clear();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<i32>() {
                Ok(_) => todo!(),
                Err(e) => println!("Error: {e}"),
            }
        }
    }
}
