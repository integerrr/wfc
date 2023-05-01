use crate::lesson::LessonListing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub username: String,
    pub enrolled_lesson: Vec<LessonListing>,
}

impl User {
    pub fn new(username: impl Into<String>) -> User {
        User {
            username: username.into(),
            enrolled_lesson: Vec::new(),
        }
    }
}
