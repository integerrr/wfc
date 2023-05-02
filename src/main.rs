mod database;
mod lesson;
mod menus;
mod user;

use std::io;

use database::Database;

fn login(db: &mut Database) {
    let mut current_username = String::new();
    println!("*******************************************");
    println!("Login username:");

    if let Err(e) = io::stdin().read_line(&mut current_username) {
        println!("Error: {e}");
    } else {
        db.login_user_validation(current_username.trim());
        current_username = current_username.trim().to_string();
        menus::main_menu(db, current_username);
    }
}

fn main() -> io::Result<()> {
    let mut menu_option = String::new();
    let mut db = database::Database::new();

    loop {
        db.clear_current_user();
        menu_option.clear();
        menus::print_login_menu();
        io::stdin().read_line(&mut menu_option)?;

        match menu_option.trim().parse::<i32>() {
            Ok(1) => login(&mut db),
            Ok(2) => break,
            Ok(_) => println!("invalid"),
            Err(e) => println!("Error: {e}"),
        }
    }
    Ok(())
}
