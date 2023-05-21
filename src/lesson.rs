use chrono::{DateTime, Local};
use std::fmt::Display;
use strum_macros::EnumIter;

use crate::user::User;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Default)]
pub enum LessonType {
    #[default]
    BoxFit,
    Spin,
    Yoga,
    Zumba,
}

impl Display for LessonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BoxFit => write!(f, "Box Fit"),
            Self::Spin => write!(f, "Spin"),
            Self::Yoga => write!(f, "Yoga"),
            Self::Zumba => write!(f, "Zumba"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LessonListing {
    date: DateTime<Local>,
    lesson_type: LessonType,
    max_student_capacity: i32,
    reviews: Vec<LessonReview>,
    students_enrolled: Vec<User>,
}

impl LessonListing {
    pub fn new(date: DateTime<Local>, lesson_type: LessonType) -> LessonListing {
        const MAX_STUDENT_CAPACITY: i32 = 5;
        LessonListing {
            date,
            lesson_type,
            max_student_capacity: MAX_STUDENT_CAPACITY,
            reviews: Vec::new(),
            students_enrolled: Vec::new(),
        }
    }
    pub fn enroll_student(&mut self, user: User) {
        self.students_enrolled.push(user);
    }

    pub fn remove_student(&mut self, user: User) {
        let student_idx = self
            .students_enrolled
            .iter()
            .position(|stu| stu.is_same_as(&user))
            .expect("exist");
        self.students_enrolled.swap_remove(student_idx);
    }

    pub fn get_date(&self) -> DateTime<Local> {
        self.date
    }

    pub fn get_lesson_type(&self) -> LessonType {
        self.lesson_type
    }

    pub fn get_reviews(&self) -> Vec<LessonReview> {
        self.reviews.clone()
    }

    pub fn get_students_enrolled(&self) -> Vec<User> {
        self.students_enrolled.clone()
    }

    pub fn get_reviews_mut(&mut self) -> &mut Vec<LessonReview> {
        &mut self.reviews
    }

    pub fn get_students_enrolled_mut(&mut self) -> &mut Vec<User> {
        &mut self.students_enrolled
    }

    pub fn get_price(&self) -> f32 {
        match self.lesson_type {
            LessonType::BoxFit => 14.17,
            LessonType::Spin => 15.28,
            LessonType::Yoga => 12.73,
            LessonType::Zumba => 13.49,
        }
    }

    pub fn get_vacancy(&self) -> i32 {
        self.max_student_capacity - (self.students_enrolled.len() as i32)
    }

    pub fn is_same_as(&self, other: &LessonListing) -> bool {
        (self.date == other.date) && (self.lesson_type == other.lesson_type)
    }

    pub fn add_review(&mut self, review: LessonReview) {
        self.reviews.push(review);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonReview {
    user: User,
    rating: LessonRating,
    comment: String,
}

impl LessonReview {
    pub fn new(user: User, rating: LessonRating, comment: String) -> LessonReview {
        LessonReview {
            user,
            rating,
            comment,
        }
    }

    pub fn get_user(&self) -> User {
        self.user.clone()
    }

    pub fn get_rating(&self) -> LessonRating {
        self.rating
    }

    pub fn get_comment(&self) -> String {
        self.comment.clone()
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LessonRating {
    VeryGood = 5,
    Good = 4,
    Ok = 3,
    Bad = 2,
    VeryBad = 1,
    #[default]
    Default = 0,
}
