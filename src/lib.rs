extern crate core;
use std::ops::Sub;
use chrono::{DateTime, Utc};
use time::PrimitiveDateTime;

//use time::macros::{date, datetime};

pub fn test() {
    let current_time = Utc::now();

    println!("{}", Utc::now().format("%d/%m/%Y %H:%M"));

    println!("Duration -1 Day {}", get_date(current_time, 1));
    println!("Duration -1 Week {}", get_date(current_time, 2));
    println!("Duration -4 Weeks {}", get_date(current_time, 3));
    println!("Duration -1 Year {}", get_date(current_time, 4));
}

//get date (calculate date)
fn get_date(date: DateTime<Utc>, unit: u8) -> DateTime<Utc> {


    match unit {
        1 => date.sub(chrono::Duration::days(1)),   //day
        2 => date.sub(chrono::Duration::weeks(1)),  //week
        3 => date.sub(chrono::Duration::weeks(4)),  //month (4 weeks)
        4 => date.sub(chrono::Duration::weeks(52)), //year (52 weeks)
        _ => panic!("Impossible value"),
    }


}
//data struct na save dat (output file?)

//struct: tep, tlak, SpO2, BMI

struct History {
    entry: Vec<OneMeasurement>,
}

struct OneMeasurement {
    heart_rate: u8,
    blood_pressure: u8,
    sp02: u8,
    weight: f64,
    height: f64,
    bmi: f64,
    date: PrimitiveDateTime,
}

impl OneMeasurement {
    fn get_bmi(&mut self) {
        self.bmi = self.weight / (self.height * self.height);
    }
}

//get bmi (calculate bmi)

//get all data on a date
