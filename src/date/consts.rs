use chrono::NaiveDate;

/// The first day of the Republican Calendar: 22 September 1792 in the Gregorian Calendar.
pub const FISRT_DAY: NaiveDate = NaiveDate::from_ymd_opt(1792, 9, 22).unwrap();

/// The number of days in a common year in the Republican Calendar.
pub const DAYS_IN_COMMON_YEAR: u32 = 365;
/// The number of days in a leap year in the Republican Calendar.
pub const DAYS_IN_LEAP_YEAR: u32 = 366;
/// The number of days in a normal month in the Republican Calendar.
pub const DAYS_IN_MONTH: u32 = 30;
/// The number of days in the Sans-culottides in a common year in the Republican Calendar.
pub const DAYS_IN_SANS_CULOTTIDES_COMMON_YEAR: u32 = 5;
/// The number of days in the Sans-culottides in a leap year in the Republican Calendar.
pub const DAYS_IN_SANS_CULOTTIDES_LEAP_YEAR: u32 = 6;
/// The number of months in a common year in the Republican Calendar (not counting Sans-culottides).
pub const NORMAL_MONTHS_IN_YEAR: u32 = 12;
/// The nominal index of the Sans-culottides month (13).
pub const SANS_CULOTTIDES_MONTH_INDEX: u32 = NORMAL_MONTHS_IN_YEAR + 1;
/// The number of days in the normal months of a common year in the Republican Calendar.
pub const DAYS_IN_NORMAL_MONTHS: u32 = NORMAL_MONTHS_IN_YEAR * DAYS_IN_MONTH;
