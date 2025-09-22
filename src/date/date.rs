use core::{
    fmt::{self},
    marker::PhantomData,
};

use chrono::NaiveDate;

use crate::date::{
    Day, DayOfYear, Month, Year,
    consts::{DAYS_IN_COMMON_YEAR, FISRT_DAY},
    leap::{DelambreRomme, LeapSystem},
};

/// The default date type in the Republican Calendar, using the Delambre-Romme leap year system.
pub type Date = DateWithLeap<DelambreRomme>;

/// A date in the Republican Calendar, parameterized by a leap year system.
///
/// [`Date`] is a type alias for [`DateWithLeap<DelambreRomme>`], which uses the Delambre-Romme
/// leap year system as the default.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateWithLeap<L: LeapSystem> {
    year: Year,
    doy: DayOfYear,
    _leap: PhantomData<L>,
}

impl<L: LeapSystem> DateWithLeap<L> {
    /// Create a new Date from a year and a day of year. Returns [`None`] if the input is invalid.
    pub fn try_from_yo(year: Year, doy: DayOfYear) -> Option<Self> {
        if doy.is_legal_for_year::<L>(year) {
            Some(Self {
                year,
                doy,
                _leap: PhantomData,
            })
        } else {
            None
        }
    }

    /// Try to create a new Date from a year, month and day of month. Returns [`None`] if the input
    /// is invalid.
    pub fn try_from_ymd(year: Year, month: impl TryInto<Month>, day: Day) -> Option<Self> {
        Self::try_from_yo(year, DayOfYear::try_from_month_day(month, day)?)
    }

    /// Get the year of the date.
    pub fn year(&self) -> Year {
        self.year
    }

    /// Get the day of year of the date.
    pub fn day_of_year(&self) -> DayOfYear {
        self.doy
    }

    /// Get the month of the date.
    pub fn month(&self) -> Month {
        self.doy.into_month_day().0
    }

    /// Get the day of the month of the date.
    pub fn day(&self) -> Day {
        self.doy.into_month_day().1
    }

    /// The 0-based index of the day in the epoch of the Republican Calendar.
    ///
    /// 1st Vendémiaire, Year I (1792-09-22 Gregorian) is day 0.
    pub fn epoch_days(&self) -> isize {
        let days_in_previous_years = L::days_in_years_between(1, self.year);
        days_in_previous_years + (self.doy as isize - 1)
    }

    /// Convert the date to a Gregorian date.
    pub fn to_gregorian(&self) -> NaiveDate {
        FISRT_DAY + chrono::Duration::days(self.epoch_days() as i64)
    }

    /// Create a Republican date from a Gregorian date.
    pub fn from_gregorian(date: NaiveDate) -> Self {
        let days_since_epoch = (date - FISRT_DAY).num_days() as isize;
        // div 365 to get a first approximation of the year
        let mut year = (days_since_epoch / DAYS_IN_COMMON_YEAR as isize + 1) as i32;

        let mut days_before_year = L::days_in_years_between(1, year);
        let mut days_after_year = days_before_year + L::days_in_year(year) as isize;

        // adjust the year until we find the correct one
        loop {
            if days_before_year <= days_since_epoch && days_since_epoch < days_after_year {
                // we got it
                let day_of_year = (days_since_epoch - days_before_year + 1) as u32;

                return Self {
                    year,
                    doy: DayOfYear::try_from(day_of_year).unwrap(),
                    _leap: PhantomData,
                };
            } else if days_since_epoch < days_before_year {
                year -= 1;
                days_after_year = days_before_year;
                days_before_year -= L::days_in_year(year) as isize;
            } else {
                // it seems impossible because we will always overestimate the year at first
                unreachable!();
            }
        }
    }

    #[cfg(feature = "now")]
    /// Get today's date in the Republican Calendar.
    pub fn today() -> Self {
        let today = chrono::Local::now().date_naive();
        Self::from_gregorian(today)
    }
}

impl<L: LeapSystem> fmt::Debug for DateWithLeap<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Date<{}> {{ year: {}, month: {}, day: {} }}",
            L::NAME,
            self.year,
            self.month() as u32,
            self.day()
        )
    }
}

impl<L: LeapSystem> fmt::Display for DateWithLeap<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} Year {}", self.day(), self.month(), self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_new_years_in_century_3() {
        for rc_year in 201..=300 {
            let rc_date = Date::try_from_ymd(rc_year, 1, 1).unwrap();
            let greg_date = rc_date.to_gregorian();
            assert_eq!(greg_date.month(), 9);
            assert_eq!(greg_date.day(), 22);
        }

        for greg_year in 1992..=2091 {
            let greg_date = NaiveDate::from_ymd_opt(greg_year, 9, 22).unwrap();
            let rc_date = Date::from_gregorian(greg_date);
            assert_eq!(rc_date.month(), Month::Vendémiaire);
            assert_eq!(rc_date.day(), 1);
        }
    }

    #[test]
    fn test_invalid_dates() {
        // Test invalid dates
        assert!(Date::try_from_ymd(1, 14, 1).is_none()); // Invalid month
        assert!(Date::try_from_ymd(1, 1, 31).is_none()); // Invalid day in regular month
        assert!(Date::try_from_ymd(1, 13, 7).is_none()); // Invalid day in Sans-culottides
    }

    #[test]
    fn test_sans_culottides() {
        // Test Sans-culottides in leap and common years
        let year1_last = Date::try_from_ymd(1, 13, 5).unwrap();
        let year2_first = Date::try_from_ymd(2, 1, 1).unwrap();

        assert_eq!(
            year2_first.to_gregorian() - year1_last.to_gregorian(),
            chrono::Duration::days(1)
        );

        let year3_last = Date::try_from_ymd(3, 13, 6).unwrap();
        let year4_first = Date::try_from_ymd(4, 1, 1).unwrap();

        assert_eq!(
            year4_first.to_gregorian() - year3_last.to_gregorian(),
            chrono::Duration::days(1)
        );
    }

    #[test]
    fn test_season_transitions() {
        // Test boundary dates for season transitions
        let autumn_start = Date::try_from_ymd(1, Month::Vendémiaire as u32, 1).unwrap();
        assert_eq!(
            autumn_start.to_gregorian(),
            NaiveDate::from_ymd_opt(1792, 9, 22).unwrap()
        );

        let winter_start = Date::try_from_ymd(1, Month::Nivôse as u32, 1).unwrap();
        assert_eq!(
            winter_start.to_gregorian(),
            NaiveDate::from_ymd_opt(1792, 12, 21).unwrap()
        );

        let spring_start = Date::try_from_ymd(1, Month::Germinal as u32, 1).unwrap();
        assert_eq!(
            spring_start.to_gregorian(),
            NaiveDate::from_ymd_opt(1793, 3, 21).unwrap()
        );

        let summer_start = Date::try_from_ymd(1, Month::Messidor as u32, 1).unwrap();
        assert_eq!(
            summer_start.to_gregorian(),
            NaiveDate::from_ymd_opt(1793, 6, 19).unwrap()
        );
    }

    #[test]
    fn test_historical_dates() {
        // Test important historical dates
        let bastille_date = NaiveDate::from_ymd_opt(1793, 7, 14).unwrap();
        let rc_bastille = Date::from_gregorian(bastille_date);
        assert_eq!(rc_bastille.month(), Month::Messidor);

        let republic_date = NaiveDate::from_ymd_opt(1792, 9, 22).unwrap();
        let rc_republic = Date::from_gregorian(republic_date);
        assert_eq!(rc_republic.year, 1);
        assert_eq!(rc_republic.month(), Month::Vendémiaire);
        assert_eq!(rc_republic.day(), 1);
    }
}
