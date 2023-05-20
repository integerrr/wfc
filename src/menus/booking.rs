use chrono::{Datelike, Weekday};
use std::io;
use strum::IntoEnumIterator;

use crate::{
    database::Database,
    lesson::{LessonListing, LessonType},
};

pub fn main_menu(db: &mut Database) {
    let mut input_option = String::new();

    loop {
        input_option.clear();
        print_main_menu();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<i32>() {
                Ok(1) => book_lesson_by_weekday_menu(db),
                Ok(2) => book_lesson_by_type_menu(db),
                Ok(3) => break,
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }
}

fn book_lesson_by_weekday_menu(db: &mut Database) {
    let mut input_option = String::new();
    let mut filtered_lessons: Vec<&LessonListing> = Vec::new();

    loop {
        input_option.clear();
        filtered_lessons.clear();
        print_booking_weekday_choice();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<i32>() {
                Ok(1) => {
                    filtered_lessons =
                        get_and_display_available_lessons_by_weekday(db, Weekday::Sat);
                    break;
                }
                Ok(2) => {
                    filtered_lessons =
                        get_and_display_available_lessons_by_weekday(db, Weekday::Sun);
                    break;
                }
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }

    let chosen_lesson = choose_lesson(filtered_lessons);
    db.book_lesson(chosen_lesson);
}

fn get_and_display_available_lessons_by_weekday(db: &Database, wd: Weekday) -> Vec<&LessonListing> {
    let current_user = db.current_user.clone().expect("e");
    let filtered_lessons: Vec<_> = db
        .lessons
        .iter()
        .filter(|lesson| {
            lesson.get_date().weekday() == wd
                && !(lesson
                    .get_students_enrolled()
                    .iter()
                    .any(|u| u.is_same_as(&current_user)))
                && lesson.get_vacancy() > 0
        })
        .collect();

    if !filtered_lessons.is_empty() {
        println!();
        println!("*******************************************");
        println!("There's no available lessons!");
        println!();
    } else {
        println!();
        println!("*******************************************");
        println!("Available lessons:");
        println!();
    }

    filtered_lessons
        .iter()
        .enumerate()
        .for_each(|(index, lesson)| {
            println!(
                "{}. {} ({})",
                index + 1,
                lesson.get_date().format("%d/%m/%Y %H:%M"),
                &wd
            );
            println!("    {}, £{}", lesson.get_lesson_type(), lesson.get_price());
        });

    filtered_lessons
}

fn book_lesson_by_type_menu(db: &mut Database) {
    let mut input_option = String::new();
    let mut filtered_lessons: Vec<&LessonListing> = Vec::new();

    loop {
        input_option.clear();
        filtered_lessons.clear();
        print_booking_type_choice();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<i32>() {
                Ok(1) => {
                    filtered_lessons =
                        get_and_display_available_lessons_by_type(db, LessonType::BoxFit);
                    break;
                }
                Ok(2) => {
                    filtered_lessons =
                        get_and_display_available_lessons_by_type(db, LessonType::Spin);
                    break;
                }
                Ok(3) => {
                    filtered_lessons =
                        get_and_display_available_lessons_by_type(db, LessonType::Yoga);
                    break;
                }
                Ok(4) => {
                    filtered_lessons =
                        get_and_display_available_lessons_by_type(db, LessonType::Zumba);
                    break;
                }
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }

    let chosen_lesson: LessonListing = choose_lesson(filtered_lessons);
    db.book_lesson(chosen_lesson);
}

fn get_and_display_available_lessons_by_type(
    db: &Database,
    lesson_type: LessonType,
) -> Vec<&LessonListing> {
    let current_username = &db.current_user.as_ref().expect("exist").get_username();
    let filtered_lessons: Vec<_> = db
        .lessons
        .iter()
        .filter(|lesson| {
            lesson.get_lesson_type() == lesson_type
                && !(lesson
                    .get_students_enrolled()
                    .iter()
                    .any(|user| &user.get_username() == current_username))
                && lesson.get_vacancy() > 0
        })
        .collect();

    if !filtered_lessons.is_empty() {
        println!();
        println!("*******************************************");
        println!("There's no available lessons!");
        println!();
    } else {
        println!();
        println!("*******************************************");
        println!("Available lessons:");
        println!();
    }

    filtered_lessons
        .iter()
        .enumerate()
        .for_each(|(index, lesson)| {
            println!(
                "{}. {} ({})",
                index + 1,
                lesson.get_date().format("%d/%m/%Y %H:%M"),
                lesson.get_date().weekday()
            );
            println!("    {}, £{}", lesson.get_lesson_type(), lesson.get_price());
        });

    filtered_lessons
}

fn choose_lesson(filtered_lessons: Vec<&LessonListing>) -> LessonListing {
    let mut input_option = String::new();
    let chosen_lesson: LessonListing;

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

    chosen_lesson
}

fn print_main_menu() {
    println!();
    println!("*******************************************");
    println!("How would you like to view the available lessons?");
    println!();
    println!("1. View by date");
    println!("2. View by type of lesson");
    println!("3. Go back");
}

fn print_booking_weekday_choice() {
    println!();
    println!("*******************************************");
    println!("View lessons on:");
    println!();
    println!("1. Saturdays");
    println!("2. Sundays");
}

fn print_booking_type_choice() {
    println!();
    println!("*******************************************");
    println!("View lessons by:");
    println!();

    for (index, lesson_type) in LessonType::iter().enumerate() {
        println!("{}. {}", index + 1, lesson_type);
    }
}
