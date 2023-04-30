mod database;
mod lesson;
mod menus;
mod printmenu;
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
        menus::run_bookings_menu(&mut users, &mut lessons);
    }
}

fn main() -> io::Result<()> {
    let mut menu_option = String::new();

    let mut users: Vec<User> = Vec::new();
    let mut lessons: Vec<LessonListing> = database::generate_lessons();

    loop {
        menu_option.clear();
        printmenu::print_main_menu();
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
