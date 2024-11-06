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

        rule mm() -> u32
            = n:$(['0'..='9']*) 
            {? n.parse().or(Err("u32")) }

        rule dd() -> u32
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
        match calendar::datetime("2024-11-23T0058") {
            Ok(date) => {
                println!("Parsed date: {}", date);
                assert_eq!(date, NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 11, 23).unwrap(),
                        NaiveTime::from_hms_opt(00, 58, 00).unwrap()
                )
                );
            }
            Err(e) => {
                eprintln!("Failed to parse date: {}", e);
            }
        }
        match calendar::minutes("180") {
            Ok(mins) => {
                println!("Parsed minutes duration: {}", mins);
                assert_eq!(mins, TimeDelta::minutes(180));
            }
            Err(e) => {
                eprintln!("Failed to parse duration: {}", e);
            }
        }
        match calendar::event("2024-11-23T1800 + 180 Birthday Party") {
            Ok(mins) => {
                println!("success");
            }
            Err(e) => {
                eprintln!("Failed: {}", e);
            }
        }
        match calendar::events(
"2024-11-23T1800 + 180 Birthday Party
2024-12-31T2359 + 1 NYE Celebration!
"
) {
            Ok(mins) => {
                println!("success");
            }
            Err(e) => {
                eprintln!("Failed: {}", e);
            }
        }
    }
