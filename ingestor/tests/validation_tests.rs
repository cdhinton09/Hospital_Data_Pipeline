use csv::Reader;
use ingestor::Patient;
use std::fs::File;

// Test data extraction from csv file.
// Function returns patient records as Reader.
pub fn test_data_extraction_from_csv() -> Result<csv::Reader<File>, Box<dyn std::error::Error>> {
    //println!("Current directory: {:?}", std::env::current_dir());
    // Open file path
    let file_path = r"../output/csv/patients.csv";
    let file = File::open(file_path)?;
    let mut records = csv::Reader::from_reader(file);
    /*
        // Variables to keep track of valid records
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
    Ok(records)
}

// Testing SSN Masking and age calculator
pub fn test_ssn_masking() -> Result<(), Box<dyn std::error::Error>> {
    let mut records = test_data_extraction_from_csv()?;

    let mut count = 0;

    for result in records.deserialize() {
        let record: Patient = result?;
        let masked_ssn = record.mask_ssn();
        let pt_age = record.get_age();

        assert!(
            masked_ssn.starts_with("XXX-XX-"),
            "SSN Masking for Patient ID: {:?}",
            record.get_id()
        );
        assert!(
            pt_age >= 0,
            "Age calculation returned a 
                negative value for Patient ID: {:?}",
            record.get_id()
        );

        if count < 5 {
            println!(
                "Patient: {:?} | AGE: {} | SSN: {}",
                record.get_id(),
                pt_age,
                masked_ssn
            );
        }
        count += 1;
    }
    assert!(count > 0, "No records were processed!");
    Ok(())
}
