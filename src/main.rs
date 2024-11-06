use chrono::{NaiveDate, NaiveDateTime};

peg::parser!{
  grammar calendar() for str {
    /*
    rule number() -> u32
        = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    pub rule list() -> Vec<u32>
        = "[" l:(number() ** ",") "]" { l }
    */
    pub rule yyyy() -> u32
        = n:$(['0'..='9']{4}) 
        {? n.parse().or(Err("u32")) }

    rule mm() -> u32
        = n:$(['0'..='9']{2}) 
        {? n.parse().or(Err("u32")) }

    rule dd() -> u32
        = n:$(['0'..='9']{2}) 
        {? n.parse().or(Err("u32")) }

    pub rule date() -> NaiveDate
        = year:yyyy() "-" month:mm() "-" day:dd() 
        {
            NaiveDate::from_ymd_opt(year as i32, month, day)
                .expect("Peg Parsing Date failed") // Convert option to result
        }
  }
}

pub fn main() {
    match calendar::yyyy("2024") {
        Ok(year) => {
            println!("Parsed year: {}", year);
        }
        Err(e) => {
            eprintln!("Failed to parse year: {}", e);
        }
    }
    /*
    match calendar::date("2024-11-23") {
        Ok(date) => {
            println!("Parsed date: {}", date);
            assert_eq!(date, NaiveDate::from_ymd_opt(2024, 11, 23).unwrap());
        }
        Err(e) => {
            eprintln!("Failed to parse date: {}", e);
        }
    }
    */
}
