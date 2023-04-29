use crate::lesson;

#[derive(Debug, Clone)]
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

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}