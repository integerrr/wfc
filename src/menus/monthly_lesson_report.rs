use std::{collections::HashSet, io};

use chrono::Datelike;

use crate::database::Database;

pub fn main_menu(db: &Database) {
    let selected_month = print_and_select_month(db);

    let lessons: Vec<_> = db
        .lessons
        .iter()
        .filter(|l| l.get_date().month() == selected_month)
        .collect();

    println!("*******************************************");
    for l in lessons {
        let customer_count = l.get_reviews().len();
        let sum: f32 = l
            .get_reviews()
            .iter()
            .map(|r| r.get_rating() as i32 as f32)
            .sum();
        let average_rating = sum / customer_count as f32;

        println!("*******************************************");
        println!(
            "{} lesson, {} ({})",
            l.get_lesson_type(),
            l.get_date().format("%d/%m/%Y %H:%M"),
            l.get_date().weekday()
        );
        println!("Number of customer: {}", customer_count);
        if customer_count > 0 {
            println!("Average rating: {}", average_rating);
        }
        println!();
    }
}

fn print_and_select_month(db: &Database) -> u32 {
    let mut input_option = String::new();

    let mut months: Vec<_> = db.lessons.iter().map(|l| l.get_date().month()).collect();
    dedup(&mut months);

    println!("*******************************************");
    println!("Please select the month you want to view:");
    println!();

    for (idx, month) in months.iter().enumerate() {
        println!("{}. {}", idx + 1, display_month(month));
    }

    loop {
        input_option.clear();

        if let Err(e) = io::stdin().read_line(&mut input_option) {
            println!("Error: {e}");
        } else if let Ok(choice) = input_option.trim().parse::<usize>() {
            if let Some(month) = months.get(choice - 1) {
                let selected_month = month;
                break *selected_month;
            } else {
                println!("invalid");
            }
        } else {
            println!("cannot parse");
        }
    }
}

fn dedup(v: &mut Vec<u32>) {
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

fn display_month(m: &u32) -> &str {
    match m {
        1 => "January",
        2 => "Febuary",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "you done fucked up to get here",
    }
}
