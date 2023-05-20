use std::io;

use chrono::Datelike;

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
                Ok(1) => change_booking_menu(db),
                Ok(2) => cancel_booking_menu(db),
                Ok(3) => break,
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }
}

fn change_booking_menu(db: &mut Database) {
    let booked_lessons = get_and_display_booked_lessons(db);

    if booked_lessons.is_empty() {
        return;
    }
    let old_lesson = select_lesson(booked_lessons);

    if booking::main_menu(db).is_some() {
        db.cancel_lesson(old_lesson);
    }
}

fn cancel_booking_menu(db: &mut Database) {
    let booked_lessons = get_and_display_booked_lessons(db);

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

fn get_and_display_booked_lessons(db: &Database) -> Vec<LessonListing> {
    let current_username = &db
        .current_user
        .as_ref()
        .expect("current user should exist")
        .get_username();
    let current_user = db
        .users
        .iter()
        .find(|user| &user.get_username() == current_username)
        .expect("user should exist");

    let enrolled_lessons = current_user.get_enrolled_lessons();
    if !enrolled_lessons.is_empty() {
        println!();
        println!("*******************************************");
        println!("Your booked lessons:");
        println!();
    } else {
        println!();
        println!("*******************************************");
        println!("You have no booked lessons!");
    }

    for (index, lesson) in enrolled_lessons.iter().enumerate() {
        println!(
            "{}. {} ({})",
            index + 1,
            lesson.get_date().format("%d/%m/%Y %H:%M"),
            lesson.get_date().weekday()
        );
        println!("    {}, £{}", lesson.get_lesson_type(), lesson.get_price());
    }
    enrolled_lessons
}
