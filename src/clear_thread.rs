use crate::state::State;
use chrono::{Datelike, Utc};

pub fn start_clear_thread() {
    let mut day_of_week = get_day_of_week();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
        let new_day = get_day_of_week();
        if day_of_week != new_day {
            State::clear();
            day_of_week = new_day;
        }
    }
}

fn get_day_of_week() -> u32 {
    Utc::today().weekday().number_from_monday()
}
