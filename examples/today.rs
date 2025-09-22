use republican_calendar::date::Date;

fn main() {
    let rc_today = Date::today();
    println!(
        "Today is {today}, which is {rc_today} ({rd_desc})",
        today = chrono::Local::now().date_naive(),
        rd_desc = rc_today.day_of_year()
    );
}
