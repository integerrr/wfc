use chrono::{DateTime, Datelike, Days, Local, Timelike, Weekday};

use crate::{
    lesson::{LessonListing, LessonType},
    user::User,
};

#[derive(Debug, Clone)]
pub struct Database {
    pub users: Vec<User>,
    pub lessons: Vec<LessonListing>,
    pub current_user: Option<User>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            users: Vec::new(),
            lessons: Self::generate_lessons(),
            current_user: None,
        }
    }

    pub fn clear_current_user(&mut self) {
        self.current_user = None;
    }

    fn set_current_user(&mut self, user: User) {
        self.current_user = Some(user);
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn generate_lessons() -> Vec<LessonListing> {
        let mut lessons: Vec<LessonListing> = Vec::new();
        let mut now = Local::now();
        now = Self::change_datetime_to_10am(now);

        for day_count in 0..16 {
            now = Self::change_datetime_to_10am(now);
            let lesson_type = match day_count % 4 {
                0 => LessonType::BoxFit,
                1 => LessonType::Spin,
                2 => LessonType::Yoga,
                3 => LessonType::Zumba,
                _ => panic!("your day count maths has gone really wrong"),
            };
            if now.weekday() == Weekday::Sat || now.weekday() == Weekday::Sun {
                let new_lesson = LessonListing::new(now, lesson_type);
                lessons.push(new_lesson);

                now = now.with_hour(16).unwrap();

                let new_lesson = LessonListing::new(now, lesson_type);
                lessons.push(new_lesson);

                now = now.checked_add_days(Days::new(1)).unwrap();
            }
            while !(now.weekday() == Weekday::Sat || now.weekday() == Weekday::Sun) {
                now = now.checked_add_days(Days::new(1)).unwrap();
            }
        }

        lessons
    }

    pub fn login_user_validation(&mut self, login_username: impl Into<String>) {
        let username = login_username.into();

        if let Some(user) = self.users.iter().find(|user| user.username == username) {
            self.set_current_user(user.clone());
            println!("*******************************************");
            println!("Welcome back {}!", &username);
        } else {
            let new_user = User::new(&username);
            self.set_current_user(new_user.clone());
            self.add_user(new_user);
            println!("*******************************************");
            println!("Welcome first timer! {}", &username);
        }
        dbg!(&self.current_user);
    }

    pub fn book_lesson(&mut self, chosen_lesson: LessonListing) {
        let validated_lesson: LessonListing;

        if let Some(lesson) = self
            .lessons
            .iter_mut()
            .find(|lesson| *lesson == &chosen_lesson)
        {
            validated_lesson = lesson.clone();

            if let Some(user) = self
                .users
                .iter_mut()
                .find(|user| user.username == self.current_user.map(|user| user.username).unwrap())
            {
                user.enrolled_lesson.push(validated_lesson.clone());
                lesson.students_enrolled.push(user.clone());

                println!();
                println!(
                    "You have successfully booked a {} lesson on {} for £{}!",
                    &validated_lesson.lesson_type,
                    &validated_lesson.date.format("%d/%m/%Y %H:%M"),
                    &validated_lesson.get_price(),
                );
            } else {
                panic!("couldn't find the current user in the database??");
            }
        } else {
            panic!("couldn't find the selected lesson in database");
        }
    }

    fn change_datetime_to_10am(mut time: DateTime<Local>) -> DateTime<Local> {
        time = time
            .with_hour(10)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();
        time
    }
}
