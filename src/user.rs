use crate::lesson::LessonListing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    username: String,
    enrolled_lessons: Vec<LessonListing>,
}

impl User {
    pub fn new(username: impl Into<String>) -> User {
        User {
            username: username.into(),
            enrolled_lessons: Vec::new(),
        }
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_enrolled_lessons(&self) -> Vec<LessonListing> {
        self.enrolled_lessons.clone()
    }

    pub fn get_enrolled_lessons_mut(&mut self) -> &mut Vec<LessonListing> {
        &mut self.enrolled_lessons
    }

    pub fn add_new_lesson(&mut self, lesson: LessonListing) {
        self.enrolled_lessons.push(lesson);
    }

    pub fn remove_lesson(&mut self, lesson: LessonListing) {
        let lesson_idx = self
            .enrolled_lessons
            .iter()
            .position(|l| l.is_same_as(&lesson))
            .expect("exist");
        self.enrolled_lessons.swap_remove(lesson_idx);
    }

    pub fn is_same_as(&self, other: &User) -> bool {
        self.username == other.username
    }
}
