use std::io;

use crate::{
    database::Database,
    lesson::{LessonListing, LessonRating, LessonReview},
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
                Ok(1) => attend_lesson(db),
                Ok(2) => break,
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }
    }
}

fn attend_lesson(db: &mut Database) {
    let booked_lessons = db.get_and_display_booked_lessons();
    if booked_lessons.is_empty() {
        return;
    }

    let selected_lesson = select_lesson(booked_lessons);
    let review = get_review(db);
    db.add_lesson_review(selected_lesson, review);
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

fn get_review(db: &mut Database) -> LessonReview {
    let mut input_option = String::new();
    let mut rating = LessonRating::default();
    let mut comment = String::new();

    loop {
        input_option.clear();
        print_rating_options();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            match input_option.trim().parse::<i32>() {
                Ok(1) => rating = LessonRating::VeryBad,
                Ok(2) => rating = LessonRating::Bad,
                Ok(3) => rating = LessonRating::Ok,
                Ok(4) => rating = LessonRating::Good,
                Ok(5) => rating = LessonRating::VeryGood,
                Ok(_) => println!("invalid"),
                Err(e) => println!("Error: {e}"),
            }
        }

        if rating != LessonRating::default() {
            input_option.clear();
            break;
        }
    }

    loop {
        input_option.clear();
        println!("Please enter you comment to the lesson:");

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else {
            comment = input_option.to_string();
        }

        if !comment.is_empty() {
            break;
        }
    }
    db.build_lesson_review(rating, comment)
}

fn print_main_menu() {
    println!("*******************************************");
    println!();
    println!("1. Attend a lesson and leave a review");
    println!("2. Go back")
}

fn print_rating_options() {
    println!("*******************************************");
    println!();
    println!("1. Very bad");
    println!("2. Bad");
    println!("3. Ok");
    println!("4. Good");
    println!("5. Very good");
}
