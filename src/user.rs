use crate::lesson;

pub struct User {
    username: String,
    enrolled_lesson: Vec<lesson::LessonType>,
}

impl User {
    fn new(username: impl ToString) -> User {
        User {
            username: username.to_string(),
            enrolled_lesson: Vec::new(),
        }
    }
}