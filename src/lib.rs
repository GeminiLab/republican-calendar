//! A Republican Calendar library for Rust.
//!
//! This library provides functionality to work with the French Republican Calendar,
//! including date representation, conversion between Gregorian and Republican dates,
//! and support for different leap year systems.
//!
//! # Examples
//!
//! ```
//! use republican_calendar::date::Date;
//!
//! let rd = Date::try_from_ymd(2, 1, 1).unwrap(); // 1 Vendémiaire, Year 2
//! let gd = rd.to_gregorian(); // Corresponding Gregorian date
//! assert_eq!(gd, chrono::NaiveDate::from_ymd_opt(1793, 9, 22).unwrap());
//! ```
#![no_std]

/// Date types and utilities for the dates in the Republican Calendar.
pub mod date;
