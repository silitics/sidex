//! The `timestamp` and `date` types.

use core::fmt;
use core::str::FromStr;

use crate::types::ParseError;

/// Instant in time with nanosecond precision.
///
/// Supported instants range from `-009999-01-02T01:59:59Z` to
/// `9999-12-30T22:00:00.999999999Z`, like with the `sidex` crate. The text representation
/// is RFC 3339 in UTC, for instance, `2026-10-12T08:30:00Z`, with fractional seconds when
/// needed. Years before `0000` use the extended ISO 8601 form, for instance,
/// `-000044-03-15T12:00:00Z`.
///
/// Parsing accepts RFC 3339 timestamps with any offset, also with a space or a
/// lowercase `t` between date and time, a comma before fractional seconds, and
/// offsets without a colon or minutes. A leap second becomes the second before
/// it.
///
/// ```
/// use sidex_embedded::types::Timestamp;
///
/// let timestamp: Timestamp = "2026-10-12T10:30:00.5+02:00".parse().unwrap();
/// assert_eq!(timestamp.unix_seconds(), 1_791_793_800);
/// assert_eq!(timestamp.subsec_nanos(), 500_000_000);
/// assert_eq!(timestamp.to_string(), "2026-10-12T08:30:00.5Z");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Timestamp {
    seconds: i64,
    nanos: u32,
}

/// Unix seconds of the first supported instant.
const MIN_SECONDS: i64 = -377_705_023_201;
/// Unix seconds of the last supported second.
const MAX_SECONDS: i64 = 253_402_207_200;

impl Timestamp {
    /// The Unix epoch, `1970-01-01T00:00:00Z`.
    pub const UNIX_EPOCH: Timestamp = Timestamp {
        seconds: 0,
        nanos: 0,
    };

    /// Creates a timestamp from whole seconds and nanoseconds since the Unix
    /// epoch, or returns `None` if the instant is out of range.
    ///
    /// The nanoseconds must be less than one second and are added to the
    /// seconds, so `-1` seconds and `500_000_000` nanoseconds is half a second
    /// before the epoch.
    pub const fn from_unix(seconds: i64, nanos: u32) -> Option<Self> {
        if nanos >= 1_000_000_000 || seconds < MIN_SECONDS || seconds > MAX_SECONDS {
            return None;
        }
        Some(Self { seconds, nanos })
    }

    /// Whole seconds since the Unix epoch, rounded towards negative infinity.
    pub const fn unix_seconds(&self) -> i64 {
        self.seconds
    }

    /// Nanoseconds after [`unix_seconds`](Self::unix_seconds).
    pub const fn subsec_nanos(&self) -> u32 {
        self.nanos
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date = Date::from_days(self.seconds.div_euclid(SECONDS_PER_DAY));
        let seconds = self.seconds.rem_euclid(SECONDS_PER_DAY);
        write!(
            f,
            "{date}T{:02}:{:02}:{:02}",
            seconds / 3600,
            seconds / 60 % 60,
            seconds % 60
        )?;
        if self.nanos != 0 {
            let (mut fraction, mut digits) = (self.nanos, 9);
            while fraction % 10 == 0 {
                fraction /= 10;
                digits -= 1;
            }
            write!(f, ".{fraction:0digits$}")?;
        }
        f.write_str("Z")
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl FromStr for Timestamp {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, ParseError> {
        let mut cursor = Cursor(text.as_bytes());
        parse_timestamp(&mut cursor)
            .filter(|_| cursor.0.is_empty())
            .ok_or(ParseError::new("RFC 3339 timestamp"))
    }
}

/// Calendar date without a time zone.
///
/// Supported years range from `-9999` to `9999`. The text representation is
/// `YYYY-MM-DD`, for instance, `2026-10-12`. Years before `0000` use the
/// extended ISO 8601 form, for instance, `-000044-03-15`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i16,
    month: u8,
    day: u8,
}

impl Date {
    /// Creates a date, or returns `None` if the date does not exist or is out
    /// of range.
    pub const fn new(year: i16, month: u8, day: u8) -> Option<Self> {
        if year < -9999 || year > 9999 || month < 1 || month > 12 || day < 1 {
            return None;
        }
        if day > days_in_month(year, month) {
            return None;
        }
        Some(Self { year, month, day })
    }

    /// Year.
    pub const fn year(&self) -> i16 {
        self.year
    }

    /// Month, from `1` to `12`.
    pub const fn month(&self) -> u8 {
        self.month
    }

    /// Day of the month, starting at `1`.
    pub const fn day(&self) -> u8 {
        self.day
    }

    /// Returns the number of days since `1970-01-01`.
    ///
    /// This is the algorithm `days_from_civil` by Howard Hinnant.
    fn days(&self) -> i64 {
        let month = i64::from(self.month);
        let year = i64::from(self.year) - i64::from(month <= 2);
        let era = year.div_euclid(400);
        let year_of_era = year.rem_euclid(400);
        let day_of_year = (153 * ((month + 9) % 12) + 2) / 5 + i64::from(self.day) - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    /// Returns the date the given number of days after `1970-01-01`, which must
    /// be in range.
    ///
    /// This is the algorithm `civil_from_days` by Howard Hinnant.
    fn from_days(days: i64) -> Self {
        let days = days + 719_468;
        let era = days.div_euclid(146_097);
        let day_of_era = days.rem_euclid(146_097);
        let year_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let shifted_month = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
        let month = (shifted_month + 2) % 12 + 1;
        let year = year_of_era + era * 400 + i64::from(month <= 2);
        Self {
            year: i16::try_from(year).expect("supported instants are in supported years"),
            month: month as u8,
            day: day as u8,
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.year < 0 {
            write!(f, "-{:06}", self.year.unsigned_abs())?;
        } else {
            write!(f, "{:04}", self.year)?;
        }
        write!(f, "-{:02}-{:02}", self.month, self.day)
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl FromStr for Date {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, ParseError> {
        let mut cursor = Cursor(text.as_bytes());
        parse_date(&mut cursor)
            .filter(|_| cursor.0.is_empty())
            .ok_or(ParseError::new("date"))
    }
}

impl_text!(Timestamp, "an RFC 3339 timestamp", 64);
impl_text!(Date, "a date", 64);

const SECONDS_PER_DAY: i64 = 86_400;

const fn days_in_month(year: i16, month: u8) -> u8 {
    match month {
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn parse_timestamp(cursor: &mut Cursor<'_>) -> Option<Timestamp> {
    let date = parse_date(cursor)?;
    if !(cursor.eat(b'T') || cursor.eat(b't') || cursor.eat(b' ')) {
        return None;
    }
    let hour = cursor.number(2).filter(|hour| *hour < 24)?;
    if !cursor.eat(b':') {
        return None;
    }
    let minute = cursor.number(2).filter(|minute| *minute < 60)?;
    let mut second = 0;
    let mut nanos = 0;
    if cursor.eat(b':') {
        // Leap seconds become the second before them.
        second = cursor.number(2).filter(|second| *second <= 60)?.min(59);
        if cursor.eat(b'.') || cursor.eat(b',') {
            let digits = cursor
                .0
                .iter()
                .take_while(|byte| byte.is_ascii_digit())
                .count();
            if !(1..=9).contains(&digits) {
                return None;
            }
            nanos = cursor.number(digits)? * 10_u32.pow(9 - digits as u32);
        }
    }
    let offset = if cursor.eat(b'Z') || cursor.eat(b'z') {
        0
    } else {
        let sign = if cursor.eat(b'+') {
            1
        } else if cursor.eat(b'-') {
            -1
        } else {
            return None;
        };
        let hours = cursor.number(2).filter(|hours| *hours < 24)?;
        let minutes = if cursor.eat(b':') || cursor.0.first().is_some_and(u8::is_ascii_digit) {
            cursor.number(2)?
        } else {
            0
        };
        if minutes >= 60 {
            return None;
        }
        sign * i64::from(hours * 3600 + minutes * 60)
    };
    let seconds =
        date.days() * SECONDS_PER_DAY + i64::from(hour * 3600 + minute * 60 + second) - offset;
    Timestamp::from_unix(seconds, nanos)
}

fn parse_date(cursor: &mut Cursor<'_>) -> Option<Date> {
    // Four decimal digits fit into `i16` and two into `u8`.
    let year = if cursor.eat(b'+') {
        i16::try_from(cursor.number(6)?).ok()?
    } else if cursor.eat(b'-') {
        -i16::try_from(cursor.number(6)?).ok()?
    } else {
        cursor.number(4)? as i16
    };
    if !cursor.eat(b'-') {
        return None;
    }
    let month = cursor.number(2)? as u8;
    if !cursor.eat(b'-') {
        return None;
    }
    let day = cursor.number(2)? as u8;
    Date::new(year, month, day)
}

/// Position in a text being parsed.
struct Cursor<'t>(&'t [u8]);

impl Cursor<'_> {
    fn eat(&mut self, byte: u8) -> bool {
        let matches = self.0.first() == Some(&byte);
        if matches {
            self.0 = &self.0[1..];
        }
        matches
    }

    /// Parses exactly `digits` decimal digits.
    fn number(&mut self, digits: usize) -> Option<u32> {
        let (number, rest) = self.0.split_at_checked(digits)?;
        let value = number.iter().try_fold(0, |value, digit| {
            Some(value * 10 + char::from(*digit).to_digit(10)?)
        })?;
        self.0 = rest;
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::Date;
    use super::Timestamp;

    /// Writes timestamps in UTC with the shortest fraction and parses the
    /// supported forms of RFC 3339.
    #[test]
    fn test_timestamp_text() {
        for (seconds, nanos, text) in [
            (0, 0, "1970-01-01T00:00:00Z"),
            (1, 500_000_000, "1970-01-01T00:00:01.5Z"),
            (1, 1, "1970-01-01T00:00:01.000000001Z"),
            (-1, 999_999_999, "1969-12-31T23:59:59.999999999Z"),
            (-62_200_000_000, 0, "-000002-12-17T14:13:20Z"),
            (super::MIN_SECONDS, 0, "-009999-01-02T01:59:59Z"),
            (
                super::MAX_SECONDS,
                999_999_999,
                "9999-12-30T22:00:00.999999999Z",
            ),
        ] {
            let timestamp = Timestamp::from_unix(seconds, nanos).unwrap();
            assert_eq!(timestamp.to_string(), text);
            assert_eq!(text.parse::<Timestamp>().unwrap(), timestamp);
        }
        for (text, expected) in [
            ("2026-10-12 08:30:00z", "2026-10-12T08:30:00Z"),
            ("2026-10-12t08:30:00+02:00", "2026-10-12T06:30:00Z"),
            ("2026-10-12T08:30Z", "2026-10-12T08:30:00Z"),
            ("2026-10-12T08:30:00,25-0130", "2026-10-12T10:00:00.25Z"),
            ("2026-10-12T08:30:00+02", "2026-10-12T06:30:00Z"),
            ("2026-10-12T08:30:60Z", "2026-10-12T08:30:59Z"),
            ("+002026-10-12T08:30:00Z", "2026-10-12T08:30:00Z"),
        ] {
            assert_eq!(text.parse::<Timestamp>().unwrap().to_string(), expected);
        }
        for text in [
            "2026-10-12",
            "2026-10-12T24:00:00Z",
            "2026-10-12T08:30:00",
            "2026-10-12T08:30:00.Z",
            "2026-10-12T08:30:00.1234567891Z",
            "2026-02-30T08:30:00Z",
            "9999-12-30T22:00:01Z",
            "9999-12-30T22:00:00-00:01",
            "2026-10-12T08:30:00Z ",
        ] {
            assert!(text.parse::<Timestamp>().is_err(), "{text}");
        }
    }

    /// Validates dates and writes years before `0000` in the extended form.
    #[test]
    fn test_date() {
        assert_eq!(Date::new(-44, 3, 15).unwrap().to_string(), "-000044-03-15");
        assert_eq!(
            "-000044-03-15".parse::<Date>().unwrap(),
            Date::new(-44, 3, 15).unwrap()
        );
        assert!(Date::new(2024, 2, 29).is_some());
        assert!(Date::new(2026, 2, 29).is_none());
        assert!(Date::new(1900, 2, 29).is_none());
        assert!("2026-1-01".parse::<Date>().is_err());
    }
}
