//! The `timestamp` and `date` built-in types.

use std::fmt;
use std::str::FromStr;

use super::ParseError;

/// Instant in time with nanosecond precision.
///
/// Supported instants range from the year `-9999` to the year `9999`. The text
/// representation is RFC 3339 in UTC, for instance, `2026-10-12T08:30:00Z`,
/// with fractional seconds when needed. Years before `0000` use the extended
/// ISO 8601 form, for instance, `-000044-03-15T12:00:00Z`. Parsing accepts any
/// offset and converts the instant to UTC.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(jiff::Timestamp);

impl_scalar!(Timestamp, "timestamp", Timestamp, timestamp);

impl Timestamp {
    /// The Unix epoch, `1970-01-01T00:00:00Z`.
    pub const UNIX_EPOCH: Timestamp = Timestamp(jiff::Timestamp::UNIX_EPOCH);

    /// Current instant according to the system clock.
    pub fn now() -> Self {
        Self(jiff::Timestamp::now())
    }

    /// Creates a timestamp from whole seconds and nanoseconds since the Unix
    /// epoch, or returns `None` if the instant is out of range.
    ///
    /// The nanoseconds must be less than one second and are added to the
    /// seconds, so `-1` seconds and `500_000_000` nanoseconds is half a second
    /// before the epoch.
    pub fn from_unix(seconds: i64, nanos: u32) -> Option<Self> {
        if nanos >= 1_000_000_000 {
            return None;
        }
        let total = i128::from(seconds) * 1_000_000_000 + i128::from(nanos);
        jiff::Timestamp::from_nanosecond(total).ok().map(Self)
    }

    /// Whole seconds since the Unix epoch, rounded towards negative infinity.
    pub fn unix_seconds(&self) -> i64 {
        i64::try_from(self.0.as_nanosecond().div_euclid(1_000_000_000))
            .expect("supported instants are within the range of `i64` seconds")
    }

    /// Nanoseconds after [`unix_seconds`](Self::unix_seconds).
    pub fn subsec_nanos(&self) -> u32 {
        u32::try_from(self.0.as_nanosecond().rem_euclid(1_000_000_000))
            .expect("the remainder of a division by one billion fits into `u32`")
    }
}

impl Default for Timestamp {
    /// Returns the Unix epoch.
    fn default() -> Self {
        Self::UNIX_EPOCH
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl FromStr for Timestamp {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(Self)
            .map_err(|_| ParseError::new("RFC 3339 timestamp"))
    }
}

#[cfg(feature = "jiff-02")]
impl From<jiff::Timestamp> for Timestamp {
    fn from(value: jiff::Timestamp) -> Self {
        Self(value)
    }
}

#[cfg(feature = "jiff-02")]
impl From<Timestamp> for jiff::Timestamp {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

#[cfg(feature = "jiff-02")]
impl_converted_scalar!(jiff::Timestamp, Timestamp, Timestamp, timestamp);

/// Calendar date without a time zone.
///
/// Supported years range from `-9999` to `9999`. The text representation is
/// `YYYY-MM-DD`, for instance, `2026-10-12`. Years before `0000` use the
/// extended ISO 8601 form, for instance, `-000044-03-15`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(jiff::civil::Date);

impl_scalar!(Date, "date", Date, date);

impl Date {
    /// Creates a date, or returns `None` if the date does not exist or is out
    /// of range.
    pub fn new(year: i16, month: u8, day: u8) -> Option<Self> {
        let month = i8::try_from(month).ok()?;
        let day = i8::try_from(day).ok()?;
        jiff::civil::Date::new(year, month, day).ok().map(Self)
    }

    /// Year of the date.
    pub fn year(&self) -> i16 {
        self.0.year()
    }

    /// Month of the date, from 1 to 12.
    pub fn month(&self) -> u8 {
        self.0.month().unsigned_abs()
    }

    /// Day of the month, from 1 to 31.
    pub fn day(&self) -> u8 {
        self.0.day().unsigned_abs()
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl FromStr for Date {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = || ParseError::new("date");
        // Jiff also accepts date-times and drops the time, which would make a
        // timestamp silently decode as a date.
        if s.contains(['T', 't', ' ']) {
            return Err(error());
        }
        s.parse().map(Self).map_err(|_| error())
    }
}

#[cfg(feature = "jiff-02")]
impl From<jiff::civil::Date> for Date {
    fn from(value: jiff::civil::Date) -> Self {
        Self(value)
    }
}

#[cfg(feature = "jiff-02")]
impl From<Date> for jiff::civil::Date {
    fn from(value: Date) -> Self {
        value.0
    }
}

#[cfg(feature = "jiff-02")]
impl_converted_scalar!(jiff::civil::Date, Date, Date, date);

#[cfg(test)]
mod tests {
    use super::*;

    /// Timestamps format in UTC and convert other offsets on parsing.
    #[test]
    fn timestamp_text() {
        let timestamp: Timestamp = "2026-10-12T10:30:00.5+02:00".parse().unwrap();
        assert_eq!(timestamp.to_string(), "2026-10-12T08:30:00.5Z");
        assert!("2026-10-12".parse::<Timestamp>().is_err());
        let ancient: Timestamp = "-000044-03-15T00:00:00Z".parse().unwrap();
        assert_eq!(ancient.to_string(), "-000044-03-15T00:00:00Z");
    }

    /// Unix seconds round down and nanoseconds are always non-negative.
    #[test]
    fn timestamp_unix_parts() {
        let timestamp = Timestamp::from_unix(-1, 500_000_000).unwrap();
        assert_eq!(timestamp.to_string(), "1969-12-31T23:59:59.5Z");
        assert_eq!(
            (timestamp.unix_seconds(), timestamp.subsec_nanos()),
            (-1, 500_000_000)
        );
        assert!(Timestamp::from_unix(0, 1_000_000_000).is_none());
    }

    /// Dates only accept the `YYYY-MM-DD` form.
    #[test]
    fn date_text() {
        let date: Date = "2026-10-12".parse().unwrap();
        assert_eq!((date.year(), date.month(), date.day()), (2026, 10, 12));
        assert_eq!(date.to_string(), "2026-10-12");
        assert!("2026-10-12T00:00".parse::<Date>().is_err());
        assert!("2026-02-30".parse::<Date>().is_err());
        let ancient = Date::new(-44, 3, 15).unwrap();
        assert_eq!(ancient.to_string(), "-000044-03-15");
        assert_eq!(ancient.to_string().parse::<Date>().unwrap(), ancient);
    }

    /// Jiff's types are represented like their counterparts.
    #[cfg(all(feature = "jiff-02", feature = "json"))]
    #[test]
    fn jiff_types() {
        use crate::formats::json;

        let timestamp: jiff::Timestamp = json::from_str(r#""2026-10-12T10:30:00+02:00""#).unwrap();
        assert_eq!(
            json::to_string(&timestamp).unwrap(),
            r#""2026-10-12T08:30:00Z""#
        );
        let date: jiff::civil::Date = json::from_str(r#""2026-10-12""#).unwrap();
        assert_eq!(json::to_string(&date).unwrap(), r#""2026-10-12""#);
        assert!(json::from_str::<jiff::civil::Date>(r#""2026-10-12T00:00""#).is_err());
    }
}
