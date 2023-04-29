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
    io::stdin().read_line(&mut username)?;
    
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
