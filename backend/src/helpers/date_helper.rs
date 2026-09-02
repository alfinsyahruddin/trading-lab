use chrono::{Datelike, Months, NaiveDate};

/// Calculates the backtest start and end dates based on data year and duration in months.
///
/// The simulation trades on the year following the screener data year (`year + 1`),
/// starting on January 1st and ending at the last day of the N-th month.
pub fn calculate_backtest_date_range(year: i32, duration_months: u32) -> (NaiveDate, NaiveDate) {
    let start_date = NaiveDate::from_ymd_opt(year + 1, 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, 1, 1).unwrap());

    let months = std::cmp::max(1, duration_months);
    let next_month_start = chrono::NaiveDate::checked_add_months(start_date, Months::new(months))
        .unwrap_or(start_date);
    let end_date = next_month_start.pred_opt().unwrap_or(start_date);

    (start_date, end_date)
}

/// Formats a date range into standard user-facing string e.g. "1 Jan 2026 - 31 Jan 2026".
pub fn format_backtest_date_range(start_date: NaiveDate, end_date: NaiveDate) -> String {
    format!(
        "{} - {}",
        format_date_short(start_date),
        format_date_short(end_date)
    )
}

fn format_date_short(date: NaiveDate) -> String {
    let day = date.day();
    let month_str = match date.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    };
    let year = date.year();
    format!("{} {} {}", day, month_str, year)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtest_date_range_1_month_2025() {
        let (start, end) = calculate_backtest_date_range(2025, 1);
        assert_eq!(start, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2026, 1, 31).unwrap());
        assert_eq!(
            format_backtest_date_range(start, end),
            "1 Jan 2026 - 31 Jan 2026"
        );
    }

    #[test]
    fn test_backtest_date_range_3_months_2025() {
        let (start, end) = calculate_backtest_date_range(2025, 3);
        assert_eq!(start, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2026, 3, 31).unwrap());
        assert_eq!(
            format_backtest_date_range(start, end),
            "1 Jan 2026 - 31 Mar 2026"
        );
    }

    #[test]
    fn test_backtest_date_range_6_months_2025() {
        let (start, end) = calculate_backtest_date_range(2025, 6);
        assert_eq!(start, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2026, 6, 30).unwrap());
        assert_eq!(
            format_backtest_date_range(start, end),
            "1 Jan 2026 - 30 Jun 2026"
        );
    }

    #[test]
    fn test_backtest_date_range_12_months_2025() {
        let (start, end) = calculate_backtest_date_range(2025, 12);
        assert_eq!(start, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2026, 12, 31).unwrap());
        assert_eq!(
            format_backtest_date_range(start, end),
            "1 Jan 2026 - 31 Dec 2026"
        );
    }

    #[test]
    fn test_backtest_date_range_leap_year_february() {
        // Screener year 2023 -> Backtest year 2024 (Leap year)
        let (start, end) = calculate_backtest_date_range(2023, 2);
        assert_eq!(start, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
        assert_eq!(
            format_backtest_date_range(start, end),
            "1 Jan 2024 - 29 Feb 2024"
        );
    }

    #[test]
    fn test_backtest_date_range_non_leap_year_february() {
        // Screener year 2022 -> Backtest year 2023 (Non-leap year)
        let (start, end) = calculate_backtest_date_range(2022, 2);
        assert_eq!(start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2023, 2, 28).unwrap());
        assert_eq!(
            format_backtest_date_range(start, end),
            "1 Jan 2023 - 28 Feb 2023"
        );
    }
}
