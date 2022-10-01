extern crate core;

use chrono::{Date, DateTime, Utc};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Sub;

//use time::macros::{date, datetime};

pub fn test() {
    let current_time = Utc::now();

    let utc_test: Date<Utc> = Utc::today();

    println!("Today's date (finding this one): {:?}", utc_test);

    let mut main_template_struct = History::generate_template();

    println!("Struct is:\n {:?}", main_template_struct);

    println!(
        "Result of search is:\n {:?}",
        main_template_struct.get_data_specific(&utc_test)
    );
    main_template_struct.get_bmi();

    println!("BMI filled:\n {:#?}", main_template_struct);

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
    name: &'static str,
    heart_rate: u8,
    blood_pressure_systolic: u8,
    blood_pressure_diastolic: u8,
    sp02: u8,
    weight_cm: f64,
    height_kg: f64,
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
                    name: "robo",
                    heart_rate: 55,
                    blood_pressure_systolic: 120,
                    blood_pressure_diastolic: 80,
                    sp02: 98,
                    weight_cm: 190.0,
                    height_kg: 75.3,
                    bmi: 0.0,
                    date: Utc::today(),
                },
                OneMeasurement {
                    name: "jonas",
                    heart_rate: 85,
                    blood_pressure_systolic: 140,
                    blood_pressure_diastolic: 35,
                    sp02: 100,
                    weight_cm: 170.0,
                    height_kg: 90.7,
                    bmi: 0.0,
                    date: Utc::today(),
                },
            ],
        }
    }

    fn get_bmi(&mut self) {
        self.entry.iter_mut().for_each(|x| {
            println!("BMI is now set to: {}: {}", x.name, x.bmi);

            //calculate the BMI, prepare to round to a specified decimal place
            //(*1000 for display + *100 for the next calculation)
            x.bmi = (x.weight_cm / x.height_kg.powf(2.0)) * 100000.;

            //round to a specified decimal place
            x.bmi = x.bmi .round() / 100.0;
        })
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
