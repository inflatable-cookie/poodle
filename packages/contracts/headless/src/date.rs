//! Date and calendar machinery. Mirror of core `date.ts` (the pure subset:
//! parse/format/arithmetic/ranges/calendar grid). Locale label formatting
//! (Intl on the web) stays per-runtime. Unlike the TS builder, the calendar
//! grid takes `today_iso` explicitly — the Rust core has no clock.

/// Calendar date; `month` is 1–12, `day` 1–31, validated at construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IsoDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekStart {
    Sunday,
    Monday,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarDay {
    pub iso: String,
    pub label: String,
    pub in_month: bool,
    pub is_today: bool,
}

pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Days since the Unix epoch (Howard Hinnant's civil-days algorithm).
pub fn to_epoch_days(date: IsoDate) -> i64 {
    let y = i64::from(date.year) - i64::from(date.month <= 2);
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let m = i64::from(date.month);
    let doy = ((153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(date.day) - 1) as u64;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;

    era * 146097 + doe as i64 - 719468
}

pub fn from_epoch_days(days: i64) -> IsoDate {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;

    IsoDate {
        year: (if m <= 2 { y + 1 } else { y }) as i32,
        month: m,
        day: d,
    }
}

/// Weekday, 0 = Sunday (matching JS `getUTCDay`). Epoch day 0 was a Thursday.
pub fn weekday(date: IsoDate) -> u32 {
    (to_epoch_days(date) + 4).rem_euclid(7) as u32
}

pub fn parse_iso_date(value: &str) -> Option<IsoDate> {
    let bytes = value.as_bytes();

    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }

    let year: i32 = value[0..4].parse().ok()?;
    let month: u32 = value[5..7].parse().ok()?;
    let day: u32 = value[8..10].parse().ok()?;

    if !(1..=12).contains(&month) || day < 1 || day > days_in_month(year, month) {
        return None;
    }

    Some(IsoDate { year, month, day })
}

pub fn format_iso_date(date: IsoDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

pub fn add_days(date: IsoDate, amount: i64) -> IsoDate {
    from_epoch_days(to_epoch_days(date) + amount)
}

/// Month paging: anchors to the 1st, matching the TS core.
pub fn add_months(date: IsoDate, amount: i32) -> IsoDate {
    let total = date.year * 12 + (date.month as i32 - 1) + amount;
    let year = total.div_euclid(12);
    let month = (total.rem_euclid(12) + 1) as u32;

    IsoDate { year, month, day: 1 }
}

pub fn start_of_month(date: IsoDate) -> IsoDate {
    IsoDate { day: 1, ..date }
}

pub fn month_anchor_iso(value: &str) -> Option<String> {
    parse_iso_date(value).map(|date| format_iso_date(start_of_month(date)))
}

pub fn compare_iso_date(left: &str, right: &str) -> Option<i32> {
    let l = parse_iso_date(left)?;
    let r = parse_iso_date(right)?;

    Some((to_epoch_days(l) - to_epoch_days(r)).signum() as i32)
}

/// Range endpoints ordered start <= end (mirror of `normalizeDateRange`).
pub fn normalize_date_range(start: Option<&str>, end: Option<&str>) -> (Option<String>, Option<String>) {
    match (parse_iso_date(start.unwrap_or("")), parse_iso_date(end.unwrap_or(""))) {
        (Some(s), Some(e)) if to_epoch_days(s) > to_epoch_days(e) => {
            (Some(format_iso_date(e)), Some(format_iso_date(s)))
        }
        _ => (start.map(str::to_string), end.map(str::to_string)),
    }
}

/// Inclusive range membership (both endpoints required, mirror of TS).
pub fn is_iso_date_within_range(iso: &str, start: Option<&str>, end: Option<&str>) -> bool {
    let (Some(date), Some(s), Some(e)) = (
        parse_iso_date(iso),
        start.and_then(parse_iso_date),
        end.and_then(parse_iso_date),
    ) else {
        return false;
    };

    let d = to_epoch_days(date);

    d >= to_epoch_days(s) && d <= to_epoch_days(e)
}

fn weekday_offset(day: u32, week_starts_on: WeekStart) -> u32 {
    match week_starts_on {
        WeekStart::Monday => (day + 6) % 7,
        WeekStart::Sunday => day,
    }
}

pub fn start_of_week(date: IsoDate, week_starts_on: WeekStart) -> IsoDate {
    add_days(date, -i64::from(weekday_offset(weekday(date), week_starts_on)))
}

/// Signed day delta from `iso` to its week start (negative) or end (positive).
pub fn day_delta_for_week_boundary(iso: &str, week_starts_on: WeekStart, to_end: bool) -> i32 {
    let Some(date) = parse_iso_date(iso) else {
        return 0;
    };

    let offset = weekday_offset(weekday(date), week_starts_on) as i32;

    if to_end {
        6 - offset
    } else {
        -offset
    }
}

pub fn days_between(start: &str, end: &str) -> Option<i64> {
    Some(to_epoch_days(parse_iso_date(end)?) - to_epoch_days(parse_iso_date(start)?))
}

/// Six full weeks covering the anchor month, mirror of `buildCalendarWeeks`.
/// `today_iso` is explicit — the core has no clock.
pub fn build_calendar_weeks(
    visible_month: &str,
    week_starts_on: WeekStart,
    today_iso: &str,
) -> Vec<Vec<CalendarDay>> {
    let anchor = parse_iso_date(visible_month)
        .or_else(|| parse_iso_date(today_iso))
        .map(start_of_month)
        .unwrap_or(IsoDate { year: 1970, month: 1, day: 1 });
    let first_visible = start_of_week(anchor, week_starts_on);
    let first_epoch = to_epoch_days(first_visible);

    (0..6)
        .map(|week_index| {
            (0..7)
                .map(|day_index| {
                    let date = from_epoch_days(first_epoch + week_index * 7 + day_index);
                    let iso = format_iso_date(date);

                    CalendarDay {
                        is_today: iso == today_iso,
                        label: date.day.to_string(),
                        in_month: date.month == anchor.month && date.year == anchor.year,
                        iso,
                    }
                })
                .collect()
        })
        .collect()
}
