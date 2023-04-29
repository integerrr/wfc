use crate::user;
use chrono::NaiveDateTime;

pub enum LessonType {
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

pub struct LessonListing {
    date: NaiveDateTime,
    lesson_type: LessonType,
    review: Vec<LessonReview>,
    students_enrolled: Vec<user::User>,
}

impl LessonListing {
    fn new(date: NaiveDateTime, lesson_type: LessonType) -> LessonListing {
        LessonListing { date, lesson_type, review: Vec::new(), students_enrolled: Vec::new() }
    } 
}

pub struct LessonReview {
    user: user::User,
    rating: i8,
    comment: String,
}

impl LessonReview {
    
}