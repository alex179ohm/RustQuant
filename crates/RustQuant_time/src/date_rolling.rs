// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{next_business_day, previous_business_day};
use crate::calendar::Calendar;
use std::fmt;
use time::Date;

/// Date rolling business day conventions.
///
/// From Wikipedia (<https://en.wikipedia.org/wiki/Date_rolling>):
/// """
/// In finance, date rolling occurs when a payment day or date used to
/// calculate accrued interest falls on a holiday, according to a given
/// business calendar. In this case the date is moved forward or backward in
/// time such that it falls in a business day, according with the
/// same business calendar.
/// """
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateRollingConvention {
    /// Actual: paid on the actual day, even if it is a non-business day.
    Actual,

    /// Following business day: the payment date is rolled to the next business day.
    Following,

    /// Modified following business day: the payment date is rolled to the
    /// next business day, unless doing so
    /// would cause the payment to be in the next calendar month,
    /// in which case the payment date is rolled to the previous business day.
    /// Many institutions have month-end accounting procedures that necessitate this.
    ModifiedFollowing,

    /// Previous business day: the payment date is rolled to the previous business day.
    Preceding,

    /// Modified previous business day: the payment date is rolled to the previous
    /// business day, unless doing so would cause the payment to be in the previous
    /// calendar month, in which case the payment date is rolled to the next
    /// business day. Many institutions have month-end accounting procedures
    /// that necessitate this.
    ModifiedPreceding,

    /// Modified Rolling business day: the payment date is rolled to the next
    /// business day. The adjusted week date is used for the next coupon date.
    /// So adjustments are cumulative (excluding month change).
    ModifiedRolling,
}

/// Date roller trait for rolling coupon/payment dates according to a given convention.
pub trait DateRoller {
    /// Roll the date according to the given convention.
    fn roll_date(&self, date: Date, convention: &DateRollingConvention) -> Date;

    /// Roll a list of dates according to the given convention.
    fn roll_dates(&self, dates: &[Date], convention: &DateRollingConvention) -> Vec<Date>;
}

impl<C> DateRoller for C
where
    C: Calendar,
{
    #[rustfmt::skip]
    fn roll_date(&self, date: Date, convention: &DateRollingConvention) -> Date {
        match convention {
            DateRollingConvention::Actual               => DateRollingConvention::roll_date_actual(date, self),
            DateRollingConvention::Following            => DateRollingConvention::roll_date_following(date, self),
            DateRollingConvention::ModifiedFollowing    => DateRollingConvention::roll_date_modified_following(date, self),
            DateRollingConvention::Preceding            => DateRollingConvention::roll_date_preceding(date, self),
            DateRollingConvention::ModifiedPreceding    => DateRollingConvention::roll_date_modified_preceding(date, self),
            DateRollingConvention::ModifiedRolling      => DateRollingConvention::roll_date_modified_rolling(date, self),
        }
    }

    fn roll_dates(&self, dates: &[Date], convention: &DateRollingConvention) -> Vec<Date> {
        dates
            .iter()
            .map(|&date| self.roll_date(date, convention))
            .collect()
    }
}

impl Default for DateRollingConvention {
    /// Default date rolling convention: Actual (paid on the actual day, even if it is a non-business day.)
    fn default() -> Self {
        DateRollingConvention::Actual
    }
}

impl fmt::Display for DateRollingConvention {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Actual                => write!(f, "Actual"),
            Self::Following             => write!(f, "Following"),
            Self::ModifiedFollowing     => write!(f, "Modified Following"),
            Self::Preceding             => write!(f, "Preceding"),
            Self::ModifiedPreceding     => write!(f, "Modified Preceding"),
            Self::ModifiedRolling       => write!(f, "Modified Rolling"),
        }
    }
}

impl DateRollingConvention {
    /// Adjust (roll) the date according: Actual convention.
    pub(crate) fn roll_date_actual<C: Calendar>(date: Date, _calendar: &C) -> Date {
        date
    }

    /// Adjust (roll) the date according: Following convention.
    pub(crate) fn roll_date_following<C: Calendar>(date: Date, calendar: &C) -> Date {
        next_business_day(date, calendar)
    }

    /// Adjust (roll) the date according: Modified following convention.
    pub(crate) fn roll_date_modified_following<C: Calendar>(date: Date, calendar: &C) -> Date {
        let mut new_date = next_business_day(date, calendar);

        if new_date.month() != date.month() {
            new_date = previous_business_day(date, calendar);
        }

        new_date
    }

    /// Adjust (roll) the date according: Modified preceding convention.
    pub(crate) fn roll_date_modified_preceding<C: Calendar>(date: Date, calendar: &C) -> Date {
        let mut new_date = previous_business_day(date, calendar);

        if new_date.month() != date.month() {
            new_date = next_business_day(date, calendar);
        }

        new_date
    }

    /// Adjust (roll) the date according: Modified rolling convention.
    pub(crate) fn roll_date_modified_rolling<C: Calendar>(date: Date, calendar: &C) -> Date {
        let mut new_date = date;

        while !calendar.is_business_day(new_date) {
            new_date = new_date.next_day().unwrap();
        }

        new_date
    }

    /// Adjust (roll) the date according: Preceding convention.
    pub(crate) fn roll_date_preceding<C: Calendar>(date: Date, calendar: &C) -> Date {
        previous_business_day(date, calendar)
    }
}
