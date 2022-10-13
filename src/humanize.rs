/// https://github.com/console-rs/indicatif/blob/989321af966513e05d34f9a2460fdab26e9d8143/src/format.rs
use std::fmt;
use std::time::Duration;

use number_prefix::NumberPrefix;

const SECOND: Duration = Duration::from_secs(1);
const MINUTE: Duration = Duration::from_secs(60);
const HOUR: Duration = Duration::from_secs(60 * 60);
const DAY: Duration = Duration::from_secs(24 * 60 * 60);
const WEEK: Duration = Duration::from_secs(7 * 24 * 60 * 60);
const YEAR: Duration = Duration::from_secs(365 * 24 * 60 * 60);

/// Wraps an std duration for human basic formatting.
#[derive(Debug)]
pub struct FormattedDuration(pub Duration);

/// Wraps an std duration for human readable formatting.
#[derive(Debug)]
pub struct HumanDuration(pub Duration);

/// Formats bytes for human readability
#[derive(Debug)]
pub struct HumanBytes(pub u64);

/// Formats bytes for human readability using SI prefixes
#[derive(Debug)]
pub struct DecimalBytes(pub u64);

/// Formats bytes for human readability using ISO/IEC prefixes
#[derive(Debug)]
pub struct BinaryBytes(pub u64);

/// Formats counts for human readability using commas
#[derive(Debug)]
pub struct HumanCount(pub u64);

/// Formats counts for human readability using commas for floats
#[derive(Debug)]
pub struct HumanFloatCount(pub f64);

impl fmt::Display for FormattedDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut t = self.0.as_secs();
        let seconds = t % 60;
        t /= 60;
        let minutes = t % 60;
        t /= 60;
        let hours = t % 24;
        t /= 24;
        if t > 0 {
            let days = t;
            write!(f, "{}d {:02}:{:02}:{:02}", days, hours, minutes, seconds)
        } else {
            write!(f, "{:02}:{:02}:{:02}", hours, minutes, seconds)
        }
    }
}

// `HumanDuration` should be as intuitively understandable as possible.
// So we want to round, not truncate: otherwise 1 hour and 59 minutes
// would display an ETA of "1 hour" which underestimates the time
// remaining by a factor 2.
//
// To make the precision more uniform, we avoid displaying "1 unit"
// (except for seconds), because it would be displayed for a relatively
// long duration compared to the unit itself. Instead, when we arrive
// around 1.5 unit, we change from "2 units" to the next smaller unit
// (e.g. "89 seconds").
//
// Formally:
// * for n >= 2, we go from "n+1 units" to "n units" exactly at (n + 1/2) units
// * we switch from "2 units" to the next smaller unit at (1.5 unit minus half of the next smaller unit)

impl fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut idx = 0;
        for (i, &(cur, _, _)) in UNITS.iter().enumerate() {
            idx = i;
            match UNITS.get(i + 1) {
                Some(&next) if self.0 + next.0 / 2 >= cur + cur / 2 => break,
                _ => continue,
            }
        }

        let (unit, name, alt) = UNITS[idx];
        // FIXME when `div_duration_f64` is stable
        let mut t = (self.0.as_secs_f64() / unit.as_secs_f64()).round() as usize;
        if idx < UNITS.len() - 1 {
            t = Ord::max(t, 2);
        }

        match (f.alternate(), t) {
            (true, _) => write!(f, "{}{}", t, alt),
            (false, 1) => write!(f, "{} {}", t, name),
            (false, _) => write!(f, "{} {}s", t, name),
        }
    }
}

const UNITS: &[(Duration, &str, &str)] = &[
    (YEAR, "year", "y"),
    (WEEK, "week", "w"),
    (DAY, "day", "d"),
    (HOUR, "hour", "h"),
    (MINUTE, "minute", "m"),
    (SECOND, "second", "s"),
];

impl fmt::Display for HumanBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match NumberPrefix::binary(self.0 as f64) {
            NumberPrefix::Standalone(number) => write!(f, "{:.0}B", number),
            NumberPrefix::Prefixed(prefix, number) => write!(f, "{:.2} {}B", number, prefix),
        }
    }
}

impl fmt::Display for DecimalBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match NumberPrefix::decimal(self.0 as f64) {
            NumberPrefix::Standalone(number) => write!(f, "{:.0}B", number),
            NumberPrefix::Prefixed(prefix, number) => write!(f, "{:.2} {}B", number, prefix),
        }
    }
}

impl fmt::Display for BinaryBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match NumberPrefix::binary(self.0 as f64) {
            NumberPrefix::Standalone(number) => write!(f, "{:.0}B", number),
            NumberPrefix::Prefixed(prefix, number) => write!(f, "{:.2} {}B", number, prefix),
        }
    }
}

impl fmt::Display for HumanCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;

        let num = self.0.to_string();
        let len = num.len();
        for (idx, c) in num.chars().enumerate() {
            let pos = len - idx - 1;
            f.write_char(c)?;
            if pos > 0 && pos % 3 == 0 {
                f.write_char(',')?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for HumanFloatCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;

        let num = format!("{:.4}", self.0);
        let (int_part, frac_part) = match num.split_once('.') {
            Some((int_str, fract_str)) => (int_str.to_string(), fract_str),
            None => (self.0.trunc().to_string(), ""),
        };
        let len = int_part.len();
        for (idx, c) in int_part.chars().enumerate() {
            let pos = len - idx - 1;
            f.write_char(c)?;
            if pos > 0 && pos % 3 == 0 {
                f.write_char(',')?;
            }
        }
        let frac_trimmed = frac_part.trim_end_matches('0');
        if !frac_trimmed.is_empty() {
            f.write_char('.')?;
            f.write_str(frac_trimmed)?;
        }
        Ok(())
    }
}
