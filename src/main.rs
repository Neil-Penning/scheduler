use chrono::*;

pub struct UserEvent {
    start: NaiveDateTime,
    duration: TimeDelta,
    description: String
}
peg::parser!{
    grammar calendar() for str {
        pub rule yyyy() -> u32
            = n:$(['0'..='9']*) 
            {? n.parse().or(Err("u32")) }

        pub rule mm() -> u32
            = n:$(['0'..='9']*) 
            {? n.parse().or(Err("u32")) }

        pub rule dd() -> u32
            = n:$(['0'..='9']*) 
            {? n.parse().or(Err("u32")) }

        pub rule date() -> NaiveDate
            = year:yyyy() "-" month:mm() "-" day:dd() 
            {
                NaiveDate::from_ymd_opt(year as i32, month, day)
                    .expect("Peg Parsing NaiveDate failed") // Convert option to result
            }
        pub rule time() -> NaiveTime
            = n:$(['0'..='9']*) 
            {
                let hhmm: u32 = n.parse().or(Err("u32")).expect("?"); //TODO, why or and expect?
                let mm: u32 = (hhmm % 100);
                let hh: u32 = (hhmm-mm)/100;
                //println!("NaiveTime: \"{}\", hhmm:{}, hh:{}, mm:{}", n, hhmm, hh, mm);
                return NaiveTime::from_hms_opt(hh, mm, 0)
                    .expect("Peg Parsing NaiveTime failed") // Convert option to result
                    ;
            }
        pub rule datetime() -> NaiveDateTime
            = date:date() "T" time:time()
            {
                NaiveDateTime::new(date, time)
            }
        pub rule minutes() -> TimeDelta 
            = n:$(['0'..='9']*) 
            {
                let mins: u32 = n.parse().or(Err("u32")).expect("?");
                return TimeDelta::minutes(mins as i64);
            }
        pub rule description() -> String
            = n:$([^'\n']*) { n.to_string() }

        pub rule event() -> UserEvent
            = start:datetime() " + " duration:minutes() " " description:description()
            {
                UserEvent {
                    start,
                    duration,
                    description,
                }
            }
        pub rule events() -> Vec<UserEvent>
            = event:(event() ** "\n") "\n"?
            {
                event
            }
    }
}

pub fn main() {
}
#[cfg(test)]
mod tests {
    use chrono::*;
    use super::calendar;

    #[test]
    fn test_year_parsing() {
        let year = calendar::yyyy("2024").expect("Failed to parse year");
        assert_eq!(year, 2024);
    }

    #[test]
    fn test_month_parsing() {
        let month = calendar::mm("11").expect("Failed to parse month");
        assert_eq!(month, 11);
    }

    #[test]
    fn test_day_parsing() {
        let day = calendar::dd("23").expect("Failed to parse day");
        assert_eq!(day, 23);
    }

    #[test]
    fn test_date_parsing() {
        let date = calendar::date("2024-11-23").expect("Failed to parse date");
        assert_eq!(date, NaiveDate::from_ymd_opt(2024,11,23).unwrap());
    }

    #[test]
    fn test_events_parsing() {
        match calendar::events("2024-11-23T1800 + 180 Birthday Party\n2024-12-31T2359 + 1 NYE Celebration!") {
            Ok(..) => {
                println!("success");
            }
            Err(e) => {
                eprintln!("Failed: {}", e);
            }
        }
    }
}
