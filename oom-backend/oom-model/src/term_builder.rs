use chrono::prelude::*;

pub struct TermBuilder {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl TermBuilder {
    pub fn new() -> TermBuilder {
        TermBuilder {
            start: Utc::now().naive_utc(),
            end: Utc::now().naive_utc(),
        }
    }

    pub fn days_ago(self, days: i64) -> TermBuilder {
        let start = (self.start.date() - chrono::Duration::days(days)).and_hms(0, 0, 0);

        TermBuilder {
            start: start,
            end: self.end,
        }
    }

    pub fn weeks_ago(self, weeks: i64) -> TermBuilder {
        let start = (self.start.date() - chrono::Duration::weeks(weeks)).and_hms(0, 0, 0);

        TermBuilder {
            start: start,
            end: self.end,
        }
    }

    pub fn all(self) -> TermBuilder {
        let start = NaiveDateTime::new(
            NaiveDate::from_ymd(2000, 1, 1),
            NaiveTime::from_hms(0, 0, 0),
        );

        TermBuilder {
            start: start,
            end: self.end,
        }
    }

    pub fn finish(self) -> (NaiveDateTime, NaiveDateTime) {
        (self.start, self.end)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_days_ago() {
        let (s, e) = TermBuilder::new().days_ago(10).finish();
        let sub = e - s;

        assert_eq!(sub.num_days(), 10);
    }

    #[test]
    fn test_weeks_ago() {
        let (s, e) = TermBuilder::new().weeks_ago(10).finish();
        let sub = e - s;

        assert_eq!(sub.num_weeks(), 10);
    }
}
