use std::fmt;

use jiff::{Timestamp, civil::Date, fmt::temporal::DateTimeParser};
use serde::{Deserialize, Serialize};

/// A release date with only the components the provider supplied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ReleaseDate {
    year: u16,
    month: Option<u8>,
    day: Option<u8>,
}

impl ReleaseDate {
    /// Reads a year, year/month or complete ISO date, including the date of a timestamp.
    pub fn parse(value: &str) -> Option<Self> {
        let mut parts = value.trim().split(['T', ' ']).next()?.split('-');
        let year = parts.next()?.parse().ok()?;
        let month = parts.next().map(str::parse).transpose().ok()?;
        let day = parts.next().map(str::parse).transpose().ok()?;
        if parts.next().is_some() {
            return None;
        }
        Self::from_parts(year, month, day)
    }

    /// Validates provider date components; a day requires a month, and zero is never a date.
    pub fn from_parts(year: i32, month: Option<u32>, day: Option<u32>) -> Option<Self> {
        if year <= 0 || (month.is_none() && day.is_some()) {
            return None;
        }
        let date = Date::new(
            year.try_into().ok()?,
            month.unwrap_or(1).try_into().ok()?,
            day.unwrap_or(1).try_into().ok()?,
        )
        .ok()?;
        Some(Self {
            year: date.year() as u16,
            month: month.map(|_| date.month() as u8),
            day: day.map(|_| date.day() as u8),
        })
    }

    /// Converts a provider's year without supplying a month or day.
    pub fn from_year(year: i32) -> Option<Self> {
        Self::from_parts(year, None, None)
    }

    pub fn year(self) -> u16 {
        self.year
    }

    /// The complete date, absent when its month or day was not supplied.
    pub fn full_date(self) -> Option<Date> {
        Date::new(
            self.year.try_into().ok()?,
            self.month?.try_into().ok()?,
            self.day?.try_into().ok()?,
        )
        .ok()
    }
}

impl From<Date> for ReleaseDate {
    fn from(date: Date) -> Self {
        Self {
            year: date.year() as u16,
            month: Some(date.month() as u8),
            day: Some(date.day() as u8),
        }
    }
}

impl fmt::Display for ReleaseDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.year)?;
        if let Some(month) = self.month {
            write!(f, "-{month:02}")?;
        }
        if let Some(day) = self.day {
            write!(f, "-{day:02}")?;
        }
        Ok(())
    }
}

/// Reads a provider timestamp, interpreting a missing offset or time of day as UTC.
pub fn parse_timestamp(value: &str) -> Option<Timestamp> {
    let parser = DateTimeParser::new();
    parser
        .parse_timestamp(value)
        .or_else(|_| {
            parser
                .parse_datetime(value)
                .and_then(|date| jiff::tz::TimeZone::UTC.to_timestamp(date))
        })
        .or_else(|_| {
            parser
                .parse_date(value)
                .and_then(|date| jiff::tz::TimeZone::UTC.to_timestamp(date.at(0, 0, 0, 0)))
        })
        .ok()
}
