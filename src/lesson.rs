use std::fmt::Display;

use crate::user;
use chrono::{DateTime, Local};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum LessonType {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonListing {
    pub date: DateTime<Local>,
    pub lesson_type: LessonType,
    max_student_capacity: i32,
    pub review: Vec<LessonReview>,
    pub students_enrolled: Vec<user::User>,
}

impl LessonListing {
    pub fn new(date: DateTime<Local>, lesson_type: LessonType) -> LessonListing {
        const MAX_STUDENT_CAPACITY: i32 = 5;
        LessonListing {
            date,
            lesson_type,
            max_student_capacity: MAX_STUDENT_CAPACITY,
            review: Vec::new(),
            students_enrolled: Vec::new(),
        }
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonReview {
    pub user: user::User,
    pub rating: LessonRating,
    pub comment: String,
}

impl LessonReview {}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LessonRating {
    VeryGood = 5,
    Good = 4,
    Neutral = 3,
    Bad = 2,
    VeryBad = 1,
}
