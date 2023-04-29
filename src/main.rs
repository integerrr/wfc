mod database;
mod lesson;
mod user;

use std::io;

use lesson::LessonListing;
use user::User;


fn login(mut users: &mut Vec<User>, mut lessons: &mut Vec<LessonListing>) {
    let mut username = String::new();
    println!("*******************************************");
    println!("Login username:");
    
    if let Err(e) = io::stdin().read_line(&mut username) {
        println!("Error: {e}");
    } else {
        database::login_user_validation(&mut users, username.trim());
        run_bookings_menu(&mut users, &mut lessons);
    }
    // dbg!(users);
    // dbg!(lessons);
}

fn print_main_menu() {
    println!("*******************************************");
    println!("Hello, world!");
    
    println!("Welcome to WFC booking system");
    println!("1. Login");
    println!("2. Exit");
}

fn print_bookings_menu() {
    println!();
    println!("1. Book a new lesson");
    println!("2. Change/Cancel an existing booking");
    println!("3. Leave a lesson review");
    println!("4. Generate monthly lesson report");
    println!("5. Generate monthly champion fitness type report");
    println!("6. Logout");
}

fn run_bookings_menu(mut users: &mut Vec<User>, mut lessons: &mut Vec<LessonListing>) {
    let mut option_input = String::new();

    loop {
        option_input.clear();
        print_bookings_menu();

        if let Err(e) = io::stdin().read_line(&mut option_input) {
            println!("Error: {e}");
        } else {
            match option_input.trim().parse::<u8>() {
                Ok(1) => todo!(),
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

fn main() -> io::Result<()> {
    let mut menu_option = String::new();
    
    let mut users: Vec<User> = Vec::new();
    let mut lessons: Vec<LessonListing> = database::generate_lessons();

    loop {
        menu_option.clear();
        print_main_menu();
        io::stdin().read_line(&mut menu_option)?;

        match menu_option.trim().parse::<u8>() {
            Ok(1) => login(&mut users, &mut lessons),
            Ok(2) => break,
            Ok(_) => println!("invalid"),
            Err(e) => println!("Error: {e}"),
        }
    }
    Ok(())
}
