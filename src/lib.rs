#![allow(warnings, unused)]
extern crate core;

use chrono::{Date, DateTime, Utc};
use std::borrow::{Borrow, BorrowMut};
use std::cmp::max;
use std::ops::Sub;

pub fn test() {
    let current_time = Utc::now();

    let utc_test: Date<Utc> = Utc::today();

    println!("Today's date (finding this one): {:?}", utc_test);


    //generate an empty template
    let mut main_template_struct: History = History::generate_template();


    //define new 2 entries and append them
    main_template_struct.new_entry(55, 120, 80, 98, 190.0, 75.3, Utc::today());
    main_template_struct.new_entry(
        60,
        140,
        40,
        99,
        110.,
        190.,
        Utc::today().sub(chrono::Duration::days(1)),
    );

    //Search for today's entry only
    match main_template_struct.get_data_specific(&utc_test) {
        Some(T) => println!("Result of the search is: {:?}", T),
        None => println!("Error: search term result empty."),
    };

    main_template_struct.generate_bmi();

    println!("BMI filled:\n {:#?}", main_template_struct);

    println!();

    compare_data(&main_template_struct, &utc_test);
}

///get date gets a corresponding date offset.
fn get_date(date: Date<Utc>, unit: u8) -> Date<Utc> {
    match unit {
        1 => date.sub(chrono::Duration::days(1)),   //day
        2 => date.sub(chrono::Duration::weeks(1)),  //week
        3 => date.sub(chrono::Duration::weeks(4)),  //month (4 weeks)
        4 => date.sub(chrono::Duration::weeks(52)), //year (52 weeks)
        _ => panic!("Impossible value"),
    }
}

fn compare_data(data: &History, check_date: &Date<Utc>) {
    let last_day_data = data.get_data_specific(&get_date(Utc::today(), 1));
    println!("{:?}", last_day_data);
}

#[derive(Debug, Clone, Copy)]
struct OneMeasurement {
    id: u64,
    heart_rate: u8,
    blood_pressure_systolic: u8,
    blood_pressure_diastolic: u8,
    sp_o2: u8,
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
    ///This method generates a simple template for testing.
    fn generate_template<'a>() -> History {
        History {
            entry: vec![
                OneMeasurement {
                    id: 0,
                    heart_rate: 55,
                    blood_pressure_systolic: 120,
                    blood_pressure_diastolic: 80,
                    sp_o2: 98,
                    weight_cm: 190.0,
                    height_kg: 75.3,
                    bmi: 0.0,
                    date: Utc::today().sub(chrono::Duration::weeks(1)),
                },
                OneMeasurement {
                    id: 1,
                    heart_rate: 85,
                    blood_pressure_systolic: 140,
                    blood_pressure_diastolic: 35,
                    sp_o2: 100,
                    weight_cm: 170.0,
                    height_kg: 90.7,
                    bmi: 0.0,
                    date: Utc::today(),
                },
            ],
        }
    }
    /// This method generates a new entry, automatically assigning it a new ID uint number.
    ///
    /// Since we cannot delete any entries, we don't need to handle reordering.
    ///
    /// Method also calls the generate_bmi method the BMI based on the data that is present in the entire list
    /// (regenerates the BMI data for each element).
    ///
    /// Example:
    ///
    /// ```
    /// use chrono::{Date, DateTime, Utc};
    ///
    /// let mut main_template_struct: History = History::generate_template();
    ///
    /// main_template_struct.new_entry(55, 120, 80, 98, 190.0, 75.3, Utc::today());
    ///
    /// ```
    fn new_entry(
        &mut self,
        heart_rate: u8,
        blood_pressure_systolic: u8,
        blood_pressure_diastolic: u8,
        sp_o2: u8,
        weight_cm: f64,
        height_kg: f64,
        date: Date<Utc>,
    ) {
        //TODO: append not done correctly yet.
        let get_id: Vec<u64> = self.entry.iter().map(|x| x.id).collect();

        let get_id = match get_id.into_iter().max() {
            Some(T) => T,
            None => {
                println!("Vector is empty");
                0
            }
        };

        self.entry.push(OneMeasurement {
            id: get_id + 1,
            heart_rate,
            blood_pressure_systolic,
            blood_pressure_diastolic,
            sp_o2,
            weight_cm,
            height_kg,
            bmi: 0.0,
            date,
        });

        self.generate_bmi();
    }

    fn generate_bmi(&mut self) {
        self.entry.iter_mut().for_each(|x| {
            println!("BMI is now set to: {}: {}", x.id, x.bmi);

            //calculate the BMI, prepare to round to a specified decimal place
            //(*1000 for display + *100 for the next calculation)
            x.bmi = (x.weight_cm / x.height_kg.powf(2.0)) * 100000.;

            //round to a specified decimal place
            x.bmi = x.bmi.round() / 100.0;
        })
    }

    fn get_data_specific(&self, date_to_find: &Date<Utc>) -> Option<Vec<OneMeasurement>> {
        let output_new: Vec<OneMeasurement> = self
            .entry
            .iter()
            .filter(|x| x.date == *date_to_find)
            .copied()
            .collect();

        match output_new.len() {
            0 => None,
            _ => Some(output_new),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_date;
    use chrono::{Date, Utc};
    use std::ops::Sub;

    #[test]
    fn test_date() {
        let test_date = Utc::today();
        let result = get_date(test_date, 1);

        assert_eq!(result, test_date.sub(chrono::Duration::days(1)));

        let test_date = Utc::today();
        let result = get_date(test_date, 2);

        assert_eq!(result, test_date.sub(chrono::Duration::weeks(1)));

        let test_date = Utc::today();
        let result = get_date(test_date, 3);

        assert_eq!(result, test_date.sub(chrono::Duration::weeks(4)));

        let test_date = Utc::today();
        let result = get_date(test_date, 4);

        assert_eq!(result, test_date.sub(chrono::Duration::weeks(52)));
    }

    #[test]
    fn test_search() {}
}
