use crate::user;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Copy)]
pub enum LessonType {
    BoxFit,
    Spin,
    Yoga,
    Zumba,
}

impl LessonType {
    pub fn get_price(&self) -> f32 {
        match self {
            Self::BoxFit => 14.17,
            Self::Spin => 15.28,
            Self::Yoga => 12.73,
            Self::Zumba => 13.49,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LessonListing {
    pub date: DateTime<Local>,
    pub lesson_type: LessonType,
    pub review: Vec<LessonReview>,
    pub students_enrolled: Vec<user::User>,
}

impl LessonListing {
    pub fn new(date: DateTime<Local>, lesson_type: LessonType) -> LessonListing {
        LessonListing {
            date,
            lesson_type,
            review: Vec::new(),
            students_enrolled: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LessonReview {
    pub user: user::User,
    pub rating: LessonRating,
    pub comment: String,
}

impl LessonReview {}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum LessonRating {
    VeryGood = 5,
    Good = 4,
    Neutral = 3,
    Bad = 2,
    VeryBad = 1,
}
