use strum::IntoEnumIterator;

use crate::lesson::LessonType;

pub fn print_login_menu() {
    println!();
    println!("*******************************************");
    println!("Hello, world!");

    println!("Welcome to WFC booking system");
    println!("1. Login");
    println!("2. Exit");
}

pub fn print_main_menu() {
    println!();
    println!("*******************************************");
    println!("1. Book a new lesson");
    println!("2. Change/Cancel an existing booking");
    println!("3. Leave a lesson review");
    println!("4. Generate monthly lesson report");
    println!("5. Generate monthly champion fitness type report");
    println!("6. Logout");
}

pub fn print_booking_by_view_choice() {
    println!();
    println!("*******************************************");
    println!("How would you like to view the available lessons?");
    println!();
    println!("1. View by date");
    println!("2. View by type of lesson");
    println!("3. Go back");
}

pub fn print_available_lessons_menu() {}
