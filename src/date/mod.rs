/// Constants for the Republican Calendar.
pub mod consts;
#[allow(clippy::module_inception)]
mod date;
mod doy;
/// Leap year systems for the Republican Calendar.
pub mod leap;
mod month;

pub use self::{
    date::{Date, DateWithLeap},
    doy::DayOfYear,
    month::Month,
};

/// A type alias for the year in the Republican Calendar.
pub type Year = i32;
/// A type alias for the day of month in the Republican Calendar.
pub type Day = u32;
