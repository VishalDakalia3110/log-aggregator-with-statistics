use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid datetime format".to_string());
        }

        let date_parts: Vec<&str> = parts[0].split('-').collect();
        let time_parts: Vec<&str> = parts[1].split(':').collect();

        if date_parts.len() != 3 || time_parts.len() != 3 {
            return Err("Invalid date or time format".to_string());
        }

        let year: u16 = date_parts[0].parse().map_err(|_| "Invalid year")?;
        let month: u8 = date_parts[1].parse().map_err(|_| "Invalid month")?;
        let day: u8 = date_parts[2].parse().map_err(|_| "Invalid day")?;
        let hour: u8 = time_parts[0].parse().map_err(|_| "Invalid hour")?;
        let minute: u8 = time_parts[1].parse().map_err(|_| "Invalid minute")?;
        let second: u8 = time_parts[2].parse().map_err(|_| "Invalid second")?;

        // Validation
        if !(1970..=9999).contains(&year) {
            return Err("Year out of range".to_string());
        }
        if !(1..=12).contains(&month) {
            return Err("Month out of range".to_string());
        }
        if !(1..=31).contains(&day) {
            return Err("Day out of range".to_string());
        }
        if hour > 23 {
            return Err("Hour out of range".to_string());
        }
        if minute > 59 {
            return Err("Minute out of range".to_string());
        }
        if second > 59 {
            return Err("Second out of range".to_string());
        }

        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second
        )
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        )
            .cmp(&(
                other.year,
                other.month,
                other.day,
                other.hour,
                other.minute,
                other.second,
            ))
    }
}
