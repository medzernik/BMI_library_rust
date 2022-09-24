use std::ops::Sub;
use time::{Date, PrimitiveDateTime, OffsetDateTime, UtcOffset};
use time::macros::{date, datetime};

pub fn test(){
    let date_testg: PrimitiveDateTime = datetime!(2022-01-01 13:00:55);


}


//get date (calculate date)
fn get_date(date: time::PrimitiveDateTime, unit: &str) -> PrimitiveDateTime{
    match unit {
        "day" => date.sub(time::Duration::days(1)),
        "week" => date.sub(time::Duration::weeks(1)),
        "month" => date.sub(time::Duration::weeks(4)),
        "year" => date.sub(time::Duration::weeks(52)),
        _ => date,
    }
}
//data struct na save dat (output file?)

//struct: tep, tlak, SpO2, BMI

struct History{
    entry: Vec<OneMeasurement>,
}



struct OneMeasurement {
    heart_rate: u8,
    blood_pressure: u8,
    sp02: u8,
    weight: f64,
    height: f64,
    bmi: f64,
    date: PrimitiveDateTime

}

impl OneMeasurement {
    fn get_bmi(&mut self) {
        self.bmi = self.weight / (self.height * self.height);
    }
}


//get bmi (calculate bmi)


//get all data on a date