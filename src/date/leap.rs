use crate::date::{
    Year,
    consts::{DAYS_IN_COMMON_YEAR, DAYS_IN_LEAP_YEAR},
};

/// A trait of leap year systems for the Republican Calendar.
pub trait LeapSystem {
    /// The name of the leap year system.
    const NAME: &'static str = "Unnamed Leap System";

    /// Determine if a given year is a leap year.
    fn is_leap_year(year: Year) -> bool;

    /// Count the number of leap years between two years (inclusive of start, exclusive of end).
    fn leap_years_between(start: Year, end: Year) -> usize {
        (start..end).filter(|&y| Self::is_leap_year(y)).count()
    }

    /// Get the number of days in a given year.
    fn days_in_year(year: Year) -> usize {
        (if Self::is_leap_year(year) {
            DAYS_IN_LEAP_YEAR
        } else {
            DAYS_IN_COMMON_YEAR
        }) as usize
    }

    /// Count the number of days between two years (inclusive of start, exclusive of end).
    fn days_in_years_between(start: Year, end: Year) -> isize {
        let years = end - start;
        let leap_years = Self::leap_years_between(start, end) as isize;
        years as isize * DAYS_IN_COMMON_YEAR as isize + leap_years
    }
}

/// Check if a year is a leap year in the Gregorian calendar.
fn is_gregorian_leap_year(year: Year) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else {
        year % 4 == 0
    }
}

/// Check if a year is a leap year in the Gregorian calendar, with the 4000-year rule (years
/// divisible by 4000 are not leap years).
fn is_gregorian_4000_revised_leap_year(year: Year) -> bool {
    if year % 4000 == 0 {
        false
    } else {
        is_gregorian_leap_year(year)
    }
}

/// Count the number of leap years between two years in the Gregorian calendar (inclusive of start,
/// exclusive of end).
fn count_gregorian_leap_years(start: Year, end: Year) -> usize {
    if start >= end {
        return 0;
    }

    let start = start - 1;
    let end = end - 1;
    let start_div_400 = start / 400;
    let end_div_400 = end / 400;
    let start_div_100 = start / 100;
    let end_div_100 = end / 100;
    let start_div_4 = start / 4;
    let end_div_4 = end / 4;

    ((end_div_4 - start_div_4) - (end_div_100 - start_div_100) + (end_div_400 - start_div_400))
        as usize
}

/// Count the number of leap years between two years in the Gregorian calendar with the 4000-year
/// rule (inclusive of start, exclusive of end).
fn count_gregorian_4000_revised_leap_years(start: Year, end: Year) -> usize {
    if start >= end {
        return 0;
    }

    count_gregorian_leap_years(start, end) - ((end - 1) / 4000 - (start - 1) / 4000) as usize
}

/// Check if a year is a leap year according to the historical implementation of the Republican
/// Calendar (years 3, 7, and 11 are leap years).
fn is_historical_leap_year(year: Year) -> bool {
    matches!(year, 3 | 7 | 11)
}

/// Check if a year is within the historically defined range (year 1 to year 12).
fn is_historical_defined_year(year: Year) -> bool {
    (1..=12).contains(&year)
}

/// Count the number of leap years before year `year`, according to the historical implementation
/// of the Republican Calendar (years 3, 7, and 11 are leap years).
fn historical_leap_years_count(year: Year) -> usize {
    match year {
        1 => 0,
        2 => 0,
        3 => 0,
        4 => 1,
        5 => 1,
        6 => 1,
        7 => 1,
        8 => 2,
        9 => 2,
        10 => 2,
        11 => 2,
        12 => 3,
        13 => 3,
        _ => unreachable!("year must be between 1 and 12"),
    }
}

/// Count the number of leap years before year `year`, according to a given leap year counting
/// function, but taking into account the historical definition for the first 12 years.
fn count_leap_years_with_history_fix(year: Year, counter: fn(Year, Year) -> usize) -> usize {
    match year {
        ..=0 => 0,
        1..=13 => historical_leap_years_count(year),
        _ => historical_leap_years_count(13) + counter(13, year),
    }
}

/// The Gregorian-like leap year system, with the first 12 years historically defined.
pub struct GregorianLike;

impl LeapSystem for GregorianLike {
    const NAME: &'static str = "Gregorian-like";

    fn is_leap_year(year: Year) -> bool {
        if is_historical_defined_year(year) {
            is_historical_leap_year(year)
        } else {
            is_gregorian_leap_year(year)
        }
    }

    fn leap_years_between(start: Year, end: Year) -> usize {
        count_leap_years_with_history_fix(end, count_gregorian_leap_years)
            - count_leap_years_with_history_fix(start, count_gregorian_leap_years)
    }
}

/// The leap year system proposed by Delambre. It's a modified Gregorian-like leap system, with a
/// extra rule that years divisible by 4000 are not leap years.
pub struct DelambreRomme;

impl LeapSystem for DelambreRomme {
    const NAME: &'static str = "Delambre-Romme";

    fn is_leap_year(year: i32) -> bool {
        if is_historical_defined_year(year) {
            is_historical_leap_year(year)
        } else {
            is_gregorian_4000_revised_leap_year(year)
        }
    }

    fn leap_years_between(start: Year, end: Year) -> usize {
        count_leap_years_with_history_fix(end, count_gregorian_4000_revised_leap_years)
            - count_leap_years_with_history_fix(start, count_gregorian_4000_revised_leap_years)
    }
}

/// The same as [`DelambreRomme`], but with a one year offset, so that year XV (15) is a leap year,
/// instead of year XIV (16).
pub struct DelambreRommeMinus1;

impl LeapSystem for DelambreRommeMinus1 {
    const NAME: &'static str = "Delambre-Romme-1";

    fn is_leap_year(year: i32) -> bool {
        is_gregorian_4000_revised_leap_year(year + 1)
    }

    fn leap_years_between(start: Year, end: Year) -> usize {
        let counter = |s, e| count_gregorian_4000_revised_leap_years(s + 1, e + 1);
        count_leap_years_with_history_fix(end, counter)
            - count_leap_years_with_history_fix(start, counter)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::{string::String, vec::Vec};
    use core::iter;

    use super::*;

    /// Test the leap year counting functions.
    #[test]
    fn test_leap_year_calc() {
        /// Test a leap year counting function against a predicate over a range of years.
        fn testcase(
            starts: impl Iterator<Item = Year>,
            ends: impl Iterator<Item = Year>,
            testee: fn(Year, Year) -> usize,
            leap_predicate: impl Fn(Year) -> bool,
            desc: impl Into<String>,
        ) {
            let starts: Vec<_> = starts.collect();
            let ends: Vec<_> = ends.collect();

            let starts_min = *starts.iter().min().unwrap();
            let starts_max = *starts.iter().max().unwrap();
            let ends_min = *ends.iter().min().unwrap();
            let ends_max = *ends.iter().max().unwrap();
            let years_covered_start = starts_min.min(ends_min);
            let years_covered_end = starts_max.max(ends_max);

            let leap_year_prefix_sum: Vec<_> = iter::once(0)
                .chain(
                    (years_covered_start..=years_covered_end).map(|y| leap_predicate(y) as usize),
                )
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect();

            for &start in &starts {
                for &end in &ends {
                    let expected = if start >= end {
                        0
                    } else {
                        leap_year_prefix_sum[(end - years_covered_start) as usize]
                            - leap_year_prefix_sum[(start - years_covered_start) as usize]
                    };
                    let actual = testee(start, end);
                    assert_eq!(
                        expected,
                        actual,
                        "Failed for year range {}..{} ({})",
                        start,
                        end,
                        desc.into()
                    );
                }
            }
        }

        testcase(
            1..=20000,
            1..=20000,
            count_gregorian_leap_years,
            is_gregorian_leap_year,
            "Gregorian",
        );
        testcase(
            1..=20000,
            1..=20000,
            count_gregorian_4000_revised_leap_years,
            is_gregorian_4000_revised_leap_year,
            "Gregorian with 4000-year rule",
        );
    }
}
