# Hospital_Data_Pipeline

This project is a simulated data pipeline for a medical center. It uses Rust for extraction and validation, Python for transformation and reporting, and PostgreSQL for data storage management.

Bash command for data generation:
java -jar synthea-with-dependencies.jar -p 100 "Florida" --exporter.csv.export=true --exporter.fhir.export=false
