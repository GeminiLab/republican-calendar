# Republican Calendar

[![GitHub License](https://img.shields.io/github/license/GeminiLab/republican-calendar)](/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/republican-calendar)
](https://crates.io/crates/republican-calendar)
[![docs.rs](https://img.shields.io/docsrs/republican-calendar)](https://docs.rs/republican-calendar)

A Rust library for the French Republican Calendar.

## Features

- Bi-directional conversion between Gregorian and Republican calendar dates.
- Full name and formatting support for Republican dates and months.
- No std dependency (only relies on chrono for date calculations).

## Usage

```rust
use republican_calendar::date::Date;

fn main() {
    // Get today's date in Republican calendar
    let today = chrono::Local::now().date_naive();
    let rc_today = Date::from_gregorian(today);
    println!("Today is {}", rc_today);
    
    // Create a Republican calendar date
    let date = Date::try_from_ymd(1, 1, 1).unwrap(); // 1 Vendémiaire Year 1
    let gregorian = date.to_gregorian(); // Convert to Gregorian
}
```

## About the Calendar

The French Republican Calendar uses a new era beginning on September 22, 1792. Each year consists of twelve 30-day months plus 5-6 complementary days called Sans-culottides. Each month and day has a unique name, about which you can read more [here](https://en.wikipedia.org/wiki/French_Republican_Calendar).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Development Status

This project is under development. The API may change. Contributions welcome!
