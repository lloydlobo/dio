use chrono::prelude::*;

#[derive(Debug)]
pub struct Date {
    /// Returns today `Utc` weekday.
    pub weekday_utc: i32,
    /// Returns today `Local` weekday.
    pub weekday_local: i32,
}

// ----------------------------------------------------------------------------

// [Examples](https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html)
impl Date {
    fn date_local() -> i32 {
        let now: DateTime<Local> = chrono::offset::Local::now();

        now.num_days_from_ce()
    }

    fn date_utc() -> i32 {
        let now: DateTime<Utc> = chrono::offset::Utc::now();

        now.num_days_from_ce()
    }

    pub fn new() -> Self {
        Self {
            weekday_utc: Self::date_utc(),
            weekday_local: Self::date_local(),
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::new()
    }
}

/// Return today's index for list item.
///
/// Counts the days in the proleptic Gregorian calendar, with January 1, Year 1 (CE) as day 1.
/// Wrap todays day with total count of list items.
/// Add `ii32` to avoid `0` index.
pub fn idx_today(len: usize) -> i32 {
    (Date::new().weekday_local % len as i32) + 1i32
}
