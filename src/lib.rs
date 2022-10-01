extern crate core;

use chrono::{Date, DateTime, Utc};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Sub;

//use time::macros::{date, datetime};

pub fn test() {
    let current_time = Utc::now();

    let Utc_test: Date<Utc> = Utc::today();

    println!("Today's date (finding this one): {}", Utc_test);

    let main_template_struct = History::generate_template();

    println!("Struct is:\n {:?}", main_template_struct);

    println!(
        "Result of search is:\n {:?}",
        main_template_struct.get_data_specific(&Utc_test)
    );

    println!("BMI filled:\n {:?}", main_template_struct.get_bmi());

    /*
    println!("{}", Utc::now().format("%d/%m/%Y %H:%M"));

    println!("Duration -1 Day {}", get_date(current_time, 1));
    println!("Duration -1 Week {}", get_date(current_time, 2));
    println!("Duration -4 Weeks {}", get_date(current_time, 3));
    println!("Duration -1 Year {}", get_date(current_time, 4));

     */
}

//get date (calculate date)
fn get_date(date: Date<Utc>, unit: u8) -> Date<Utc> {
    match unit {
        1 => date.sub(chrono::Duration::days(1)),   //day
        2 => date.sub(chrono::Duration::weeks(1)),  //week
        3 => date.sub(chrono::Duration::weeks(4)),  //month (4 weeks)
        4 => date.sub(chrono::Duration::weeks(52)), //year (52 weeks)
        _ => panic!("Impossible value"),
    }
}

#[derive(Debug, Clone, Copy)]
struct OneMeasurement {
    heart_rate: u8,
    blood_pressure: u8,
    sp02: u8,
    weight: f64,
    height: f64,
    bmi: f64,
    date: Date<Utc>,
}

#[derive(Debug, Clone)]
struct History {
    entry: Vec<OneMeasurement>,
}

impl History {
    ///generates a simple template for testing
    fn generate_template() -> History {
        History {
            entry: vec![
                OneMeasurement {
                    heart_rate: 55,
                    blood_pressure: 120,
                    sp02: 98,
                    weight: 58.0,
                    height: 194.3,
                    bmi: 0.0,
                    date: Utc::today(),
                },
                OneMeasurement {
                    heart_rate: 55,
                    blood_pressure: 120,
                    sp02: 98,
                    weight: 58.0,
                    height: 194.3,
                    bmi: 0.0,
                    date: Utc::today(),
                },
            ],
        }
    }

    fn get_bmi(self) {
        self.entry
            .into_iter()
            .for_each(|mut x| x.bmi = x.weight / x.height.powf(x.height))
    }

    fn get_data_specific(&self, date_to_find: &Date<Utc>) -> Vec<OneMeasurement> {
        let output_new: Vec<OneMeasurement> = self
            .entry
            .iter()
            .filter(|x| x.date == *date_to_find)
            .copied()
            .collect();

        output_new
    }
}
