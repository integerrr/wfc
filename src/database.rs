use crate::{lesson::{LessonListing, LessonType}};
use chrono::{Datelike, Weekday, Timelike, DateTime, Local, Days};

pub fn generate_lessons() -> Vec<LessonListing> {
    let mut lessons: Vec<LessonListing> = Vec::new();
    let mut now = Local::now();
    now = change_datetime_to_10am(now);
    dbg!(now);

    for day_count in 0..16 {
        now = change_datetime_to_10am(now);
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

    dbg!(&lessons);
    lessons
}

fn change_datetime_to_10am(mut time: DateTime<Local>) -> DateTime<Local> {
    time = time.with_hour(10).unwrap().with_minute(0).unwrap().with_second(0).unwrap();
    time
}