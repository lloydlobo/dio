// use chrono::Utc;
// use cron::Schedule;
// use serde::{Deserialize, Serialize};
// use std::ops::Range;
// use std::str::FromStr;

// #[derive(Debug)]
// pub(crate) struct CronJob;

// impl CronJob {
//     pub(crate) fn run_cron() {
//         //                    sec min hour day_of_month month day_of_week year.
//         let expression: &str = "0 30 9,12,15 1,15 January-December Mon,Wed,Fri 2022/1";
//         let schedule = Schedule::from_str(expression).unwrap();
//         println!("Upcoming fire times:");

//         schedule
//             .upcoming::<Utc>(Utc)
//             .take(30) // Creates an iterator that yields the first n elements, or fewer if the underlying iterator ends sooner..
//             .for_each(|datetime: chrono::DateTime<Utc>| {
//                 println!("-> {}", datetime);
//             });
//     }
// }

// /// See https://crates.io/crates/cron.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub(crate) struct CronExpression {
//     pub(crate) sec: usize,
//     pub(crate) min: usize,
//     pub(crate) hour: usize,
//     pub(crate) day_of_month: Range<usize>,
//     pub(crate) month: Range<usize>,
//     pub(crate) day_of_week: Range<usize>,
//     pub(crate) year: Range<usize>,
// }

// impl CronExpression {
//     pub(crate) fn new(
//         sec: usize,
//         min: usize,
//         hour: usize,
//         day_of_month: Range<usize>,
//         month: Range<usize>,
//         day_of_week: Range<usize>,
//         year: Range<usize>,
//     ) -> Self {
//         Self {
//             sec,
//             min,
//             hour,
//             day_of_month,
//             month,
//             day_of_week,
//             year,
//         }
//     }
// }
