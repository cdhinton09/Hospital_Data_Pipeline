//#[allow(unused_variables, unused_imports, dead_code)]
use chrono::{Datelike, Local, NaiveDate};
//use std::error::Error;
use serde::Deserialize;
use std::fs::File;
//use std::result::Result;

#[derive(Debug, Deserialize)]
struct Patient {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "BIRTHDATE")]
    birthdate: String,
    #[serde(rename = "DEATHDATE")]
    deathdate: String,
    #[serde(rename = "SSN")]
    ssn: String,
    #[serde(rename = "FIRST")]
    first_name: String,
    #[serde(rename = "MIDDLE")]
    middle_name: String,
    #[serde(rename = "LAST")]
    last_name: String,
    #[serde(rename = "SUFFIX")]
    suffix: String,
    #[serde(rename = "MARITAL")]
    martial_status: String,
    #[serde(rename = "RACE")]
    race: String,
    #[serde(rename = "ETHNICITY")]
    ethnicity: String,
    #[serde(rename = "GENDER")]
    gender: String,
    #[serde(rename = "BIRTHPLACE")]
    birthplace: String,
    #[serde(rename = "ZIP")]
    zip: String,
}

// Validation for Patient Struct
impl Patient {
    // This function returns Ok(()) if valid, or error message if not
    fn validate(&self) -> Result<(), String> {
        // Check SSN length, to include dashes
        if self.ssn.len() != 11 {
            println!("Invalid SSN for PT ID: {}", self.id);
            //return Err(format!("Invalid SSN length for Patient ID: {} ", self.id));
        }

        // Check if Birthdate is in valid date formate
        let parsed_birthdate = NaiveDate::parse_from_str(&self.birthdate, "%Y-%m-%d");
        if parsed_birthdate.is_err() {
            return Err(format!("Invalid date format for Patient ID: {}", &self.id));
        }

        // Check if Deathdate is in valid date formate
        if !self.deathdate.is_empty() {
            let parsed_deathdate = NaiveDate::parse_from_str(&self.deathdate, "%Y-%m-%d");
            if parsed_deathdate.is_err() || &self.deathdate != "" {
                return Err(format!(
                    "Invalid deathdate format for Patient ID: {}",
                    &self.id
                ));
            }
        }

        // Check for gender at birth
        match self.gender.to_uppercase().as_str() {
            "F" | "M" => Ok(()),
            _ => Err(format!("Invalid gender for Patient ID: {}", &self.id)),
        };
        Ok(())
    }

    // This  function returns a masked version of the Patients SSN for privacy
    pub fn mask_ssn(&self) -> String {
        if self.ssn.len() == 11 {
            format!("XXX-XX-{}", &self.ssn[7..])
        } else {
            "INVALID-SSN".to_string()
        }
    }

    // Calculate current age of the Patient
    pub fn get_age(&self) -> i32 {
        let birth = NaiveDate::parse_from_str(&self.birthdate, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        let now = Local::now().date_naive();
        let mut age = now.year() - birth.year();

        if now.month() < birth.month() || (now.month() == birth.month() && now.day() < birth.day())
        {
            age -= 1;
        }
        age
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Current directory: {:?}", std::env::current_dir());
    // Open file path
    let file_path = r"../output/csv/patients.csv";
    let file = File::open(file_path)?;
    let mut records = csv::Reader::from_reader(file);

    /* // Variables to keep track of valid records
        let mut valid_count = 0;
        let mut error_count = 0;

        for result in records.deserialize() {
            let record: Patient = result?;

            match record.validate() {
                Ok(_) => {
                    // Later send these records to the Transform layer
                    valid_count += 1;
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!("Validation Error: {}", e);
                }
            }
        }

        println!("Data Processing Complete");
        println!("{} records successfully validated.", valid_count);
        println!("{} records failed validated", error_count);
    */

    // Testing SSN Masking and age calculator
    let mut count = 0;
    for result in records.deserialize() {
        let record: Patient = result?;
        let masked_ssn = record.mask_ssn();
        let pt_age = record.get_age();
        println!(
            "Patient ID: {} Masked SSN: {} Patient Age: {}",
            count, masked_ssn, pt_age
        );
        count += 1;
    }
    Ok(())
}
