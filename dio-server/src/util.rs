//! `util` contains common utility functions agnostic to the project.
use anyhow::{Context, Result};
use std::env;

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
//
// .unwrap_or_else(|_| panic!("{}", format!("{} environment variable not set.", key)))
// you have declared `#[inline(always)]` on `get_env_var`. This is usually a bad idea for further
// information visit https://rust-lang.github.io/rust-clippy/master/index.html#inline_always
// #[inline(always)]
#[inline]
pub fn get_env_var(key: &str) -> Result<String> {
    env::var(key).context(format!("Could not find environment variable {key}"))
}

// mod generics {
//     fn main_run() {
//         let number_list: Vec<i32> = vec![34, 50, 25, 100, 63];
//         let Ok(result) = largest_i32(number_list) else { panic!("An error occurred in the largest_i32 function.") };
//         println!("The largest number is {}", result);

//         let number_list = vec![34, 50, 25, 100, 63];
//         let result = largest_char(number_list);
//         dbg!(result);
//     }
//     /// This method returns an ordering between `self` and `other` values if one exists.
//     /// Types whose values can be duplicated simply by copying bits.
//     fn largest<T>(list: &[T]) -> T
//     where
//         T: PartialOrd + Copy,
//     {
//         let mut largest = list[0];
//         for &item in list {
//             if item > largest {
//                 largest = item;
//             }
//         }
//         largest
//     }
//     pub struct Point<T> {
//         x: T,
//         y: T,
//     }

//     impl<T> Point<T> {
//         // Access x without using pub on it.
//         fn get_x(&self) -> &T {
//             &self.x
//         }
//     }
//     impl Point<f32> {
//         fn distance_from_origin(&self) -> f32 {
//             (self.x.powi(2) + self.y.powi(2)).sqrt()
//         }
//     }
//     struct PointDiff<T, U> {
//         x: T,
//         y: U,
//     }
//     fn run_point() {
//         let integer = Point { x: 5, y: 19 };
//         let float = Point { x: 5f32, y: 19f32 };
//         let int_float = PointDiff { x: 5i32, y: 19f32 };
//         println!("integer.x = {}", integer.get_x()); // get_x or get_distance_from_origin.
//     }
//     fn largest_i32(number_list: Vec<i32>) -> Result<i32, anyhow::Error> {
//         let mut largest = *number_list.first().unwrap();
//         for number in number_list {
//             if number > largest {
//                 largest = number;
//             }
//         }
//         Ok(largest)
//     }
//     fn largest_char(number_list: Vec<i32>) -> char {
//         let mut largest: i32 = number_list[0];
//         for number in number_list.into_iter() {
//             if number > largest {
//                 largest = number;
//             }
//         }
//         match char::from_u32(largest as u32) {
//             Some(val) => val,
//             None => panic!("{}", format!("{} is not a valid unicode", largest)),
//         }
//     }
// }
