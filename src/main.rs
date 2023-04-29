use std::io;

use chrono::NaiveDateTime;

enum LessonType {
    Aquacise,
    Bodysculpt,
    BoxFit,
    Spin,
    Yoga,
    Zumba,
}

impl LessonType {
    fn get_price(&self) -> f32 {
        match self {
            Self::Aquacise => 10.33,
            Self::Bodysculpt => 7.98,
            Self::BoxFit => 4.17,
            Self::Spin => 15.28,
            Self::Yoga => 48.73,
            Self::Zumba => 50.49,
        }
    }
}

struct LessonListing {
    date: NaiveDateTime,
    lesson_type: LessonType,
    students_enrolled: Option<Vec<User>>,
    review: Option<Vec<LessonReview>>,
}

impl LessonListing {
    fn update_capacity(&self) {

    }


}
struct User {
    username: String,
    enrolled_lesson: Vec<LessonType>,
}

impl User {
    fn new(username: String) -> User {
        User {
            username,
            enrolled_lesson: Vec::new(),
        }
    }
}

struct LessonReview {
    user: User,
    rating: i8,
    comment: String,
}

impl LessonReview {
    
}

fn login() {
    println!("Login username:");

}

fn print_main_menu() {
    println!("Hello, world!");
    
    println!("Welcome to WFC booking system");
    println!("1. Login");
    println!("2. Exit");
}

fn print_bookings_menu() {

}

fn main() -> io::Result<()> {
    let mut menu_option = String::new();

    loop {
        print_main_menu();
        io::stdin().read_line(&mut menu_option)?;

        match menu_option.chars().next() {
            Some(char) => {
                match char.to_digit(10) {
                    Some(n) => {
                        match n {
                            1 => login(),
                            2 => break,
                            _ => println!("wrong"),
                        }
                    },
                    None => println!("wrong2"),
                }
            },
            None => println!("wrong3"),
        }
    }

    Ok(())
}
