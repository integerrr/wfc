use chrono::{Datelike, Weekday};
use std::io;
use strum::IntoEnumIterator;

use crate::{
    database::Database,
    lesson::{LessonListing, LessonType},
};

pub fn new_booking_menu(db: &mut Database, current_username: String) {
    let mut input_option = String::new();

    loop {
        input_option.clear();
        print_booking_by_view_choice();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<i32>() {
                Ok(1) => book_lesson_by_weekday_menu(db, current_username.clone()),
                Ok(2) => book_lesson_by_type_menu(db, current_username.clone()),
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

    loop {
        input_option.clear();
        filtered_lessons.clear();
        print_booking_weekday_choice();

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

    let chosen_lesson = choose_lesson(filtered_lessons);
    db.book_lesson(chosen_lesson, current_username);
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

fn book_lesson_by_type_menu(db: &mut Database, current_username: String) {
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
                    filtered_lessons = get_and_display_available_lessons_by_type(
                        db,
                        LessonType::BoxFit,
                        current_username.clone(),
                    );
                    break;
                }
                Ok(2) => {
                    filtered_lessons = get_and_display_available_lessons_by_type(
                        db,
                        LessonType::Spin,
                        current_username.clone(),
                    );
                    break;
                }
                Ok(3) => {
                    filtered_lessons = get_and_display_available_lessons_by_type(
                        db,
                        LessonType::Yoga,
                        current_username.clone(),
                    );
                    break;
                }
                Ok(4) => {
                    filtered_lessons = get_and_display_available_lessons_by_type(
                        db,
                        LessonType::Zumba,
                        current_username.clone(),
                    );
                    break;
                }
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }

    let chosen_lesson: LessonListing = choose_lesson(filtered_lessons);
    db.book_lesson(chosen_lesson, current_username);
}

fn get_and_display_available_lessons_by_type(
    db: &Database,
    lesson_type: LessonType,
    current_username: String,
) -> Vec<&LessonListing> {
    let filtered_lesson: Vec<_> = db
        .lessons
        .iter()
        .filter(|lesson| {
            lesson.lesson_type == lesson_type
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

    filtered_lesson
        .iter()
        .enumerate()
        .for_each(|(index, lesson)| {
            println!(
                "{}. {} ({})",
                index + 1,
                lesson.date.format("%d/%m/%Y %H:%M"),
                lesson.date.weekday()
            );
            println!("    {}, £{}", lesson.lesson_type, lesson.get_price());
        });

    filtered_lesson
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

fn print_booking_by_view_choice() {
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
