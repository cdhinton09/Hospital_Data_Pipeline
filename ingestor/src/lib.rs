// This file will house Structs and methods
use chrono::{Datelike, Local, NaiveDate};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Patient {
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
    pub fn validate(&self) -> Result<(), String> {
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

    pub fn get_id(&self) {
        &self.id;
    }
}
