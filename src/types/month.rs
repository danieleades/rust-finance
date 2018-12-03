use chrono::Datelike;
use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CalendarMonth {
    year: i32,
    month: Month,
}

impl CalendarMonth {
    pub fn new(year: i32, month: Month) -> CalendarMonth {
        CalendarMonth { year, month }
    }

    pub fn increment(&mut self) {
        match self.month {
            Month::Dec => {
                self.year += 1;
                self.month.increment();
            }
            _ => self.month.increment(),
        }
    }
}

impl<T: Datelike> From<T> for CalendarMonth {
    fn from(date: T) -> Self {
        CalendarMonth {
            year: date.year(),
            month: date.month().into(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Debug, Clone)]
pub enum Month {
    Jan=1,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    pub fn increment(&mut self) {
        *self = ((self.clone() as u32 + 1) % 12).into()
    }
}

impl<T: Into<u32>> From<T> for Month {
    fn from(m: T) -> Month {
        match m.into() {
            1 => Month::Jan,
            2 => Month::Feb,
            3 => Month::Mar,
            4 => Month::Apr,
            5 => Month::May,
            6 => Month::Jun,
            7 => Month::Jul,
            8 => Month::Aug,
            9 => Month::Sep,
            10 => Month::Oct,
            11 => Month::Nov,
            12 => Month::Dec,
            _ => panic!("there are only 12 months you dingus!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn less_than() {
        assert!(
            CalendarMonth {
                year: 2018,
                month: Month::Jan
            } < CalendarMonth {
                year: 2018,
                month: Month::Feb
            }
        );
        assert!(
            CalendarMonth {
                year: 2018,
                month: Month::Jan
            } < CalendarMonth {
                year: 2019,
                month: Month::Jan
            }
        );
    }

    #[test]
    fn increment_month() {
        let mut m = Month::Jan;
        m.increment();
        assert_eq!(m, Month::Feb);
        m = Month::Dec;
        m.increment();
        assert_eq!(m, Month::Jan);
    }
}
