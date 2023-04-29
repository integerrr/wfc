use crate::lesson;

pub struct User {
    pub username: String,
    pub enrolled_lesson: Vec<lesson::LessonType>,
}

impl User {
    pub fn new(username: impl Into<String>) -> User {
        User {
            username: username.into(),
            enrolled_lesson: Vec::new(),
        }
    }
}