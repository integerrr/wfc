mod booking;
mod change_or_cancel;

use std::io;

use crate::database::Database;

pub fn main_menu(db: &mut Database) {
    let mut option_input = String::new();
    dbg!(&db);

    loop {
        option_input.clear();
        print_main_menu();

        if let Err(e) = io::stdin().read_line(&mut option_input) {
            println!("Error: {e}");
        } else {
            match option_input.trim().parse::<i32>() {
                Ok(1) => _ = booking::main_menu(db),
                Ok(2) => change_or_cancel::main_menu(db),
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
