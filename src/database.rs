use chrono::{DateTime, Datelike, Days, Local, Timelike, Weekday};

use crate::{
    lesson::{LessonListing, LessonRating, LessonReview, LessonType},
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

        if let Some(user) = self
            .users
            .iter()
            .find(|user| user.get_username() == username)
        {
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
    }

    pub fn book_lesson(&mut self, chosen_lesson: LessonListing) {
        let curr_user = self.current_user.clone().expect("exist");
        let db_user = self
            .users
            .iter_mut()
            .find(|u| u.is_same_as(&curr_user))
            .expect("exist");
        let lesson = self
            .lessons
            .iter_mut()
            .find(|lesson| lesson.is_same_as(&chosen_lesson))
            .expect("chosen_lesson should be confirmed to exist already");

        db_user.add_new_lesson(chosen_lesson.clone());
        lesson.enroll_student(db_user.clone());

        println!();
        println!(
            "You have successfully booked a {} lesson on {} for £{}!",
            &chosen_lesson.get_lesson_type(),
            &chosen_lesson.get_date().format("%d/%m/%Y %H:%M"),
            &chosen_lesson.get_price(),
        );
    }

    pub fn cancel_lesson(&mut self, chosen_lesson: LessonListing) {
        let curr_user = self.current_user.clone().expect("exist");
        let db_user = self
            .users
            .iter_mut()
            .find(|u| u.is_same_as(&curr_user))
            .expect("exist");
        let lesson = self
            .lessons
            .iter_mut()
            .find(|l| l.is_same_as(&chosen_lesson))
            .expect("exist");

        db_user.remove_lesson(chosen_lesson.clone());
        lesson.remove_student(db_user.clone());

        println!();
        println!(
            "You have sucessfully cancelled the {} lesson on {}!",
            &chosen_lesson.get_lesson_type(),
            &chosen_lesson.get_date().format("%d/%m/%Y %H:%M"),
        );
    }

    pub fn get_and_display_booked_lessons(&self) -> Vec<LessonListing> {
        let curr_user = self.current_user.as_ref().expect("exist");
        let db_user = self
            .users
            .iter()
            .find(|u| u.is_same_as(curr_user))
            .expect("exist");

        let enrolled_lessons = db_user.get_enrolled_lessons();
        if !enrolled_lessons.is_empty() {
            println!("*******************************************");
            println!("Your booked lessons:");
            println!();
        } else {
            println!("*******************************************");
            println!("You have no booked lessons!");
        }

        for (index, lesson) in enrolled_lessons.iter().enumerate() {
            println!(
                "{}. {} ({})",
                index + 1,
                lesson.get_date().format("%d/%m/%Y %H:%M"),
                lesson.get_date().weekday()
            );
            println!("    {}, £{}", lesson.get_lesson_type(), lesson.get_price());
        }
        enrolled_lessons
    }

    pub fn build_lesson_review(&mut self, rating: LessonRating, comment: String) -> LessonReview {
        let u = self.current_user.as_ref().expect("exist");
        LessonReview::new(u.clone(), rating, comment)
    }

    pub fn add_lesson_review(&mut self, l: LessonListing, review: LessonReview) {
        let lesson = self
            .lessons
            .iter_mut()
            .find(|lesson| lesson.is_same_as(&l))
            .expect("exist");
        lesson.add_review(review);
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
