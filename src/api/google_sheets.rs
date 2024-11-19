use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

const BASE_URL: &str = "https://script.google.com/macros/s/AKfycbyHV-qBnm9IqH6_FsZQ8YDMjenGy0fvBHOhJ9nkrhm-4QZ_ko-HBUcsi1VsLXKPFJZ9zg/exec";

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyReading {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Time")]
    time: String,
    #[serde(rename = "Corrente")]
    current: f64,
}

impl EnergyReading {
    pub fn parse_timestamp(&self) -> Option<DateTime<Utc>> {
        let date_part = match DateTime::parse_from_rfc3339(&self.date) {
            Ok(parsed_date) => parsed_date.date_naive(),
            Err(_) => return None,
        };

        let time_part = match NaiveDateTime::parse_from_str(&self.time, "%Y-%m-%dT%H:%M:%S%.fZ") {
            Ok(parsed_time) => parsed_time.time(),
            Err(_) => return None,
        };

        let naive_datetime = date_part.and_time(time_part);

        Utc.from_local_datetime(&naive_datetime).earliest()
    }

    pub fn current(&self) -> f64 {
        self.current
    }
}

#[derive(Debug)]
pub struct EnergyApi {
    client: Client,
    base_url: String,
}

impl EnergyApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub async fn get_last_reading(&self) -> Result<EnergyReading, Box<dyn Error>> {
        let url = format!("{}?action=getLast", self.base_url);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<EnergyReading>()
            .await?;

        Ok(response)
    }

    pub async fn get_all_readings(&self) -> Result<Vec<EnergyReading>, Box<dyn Error>> {
        let url = format!("{}?action=getAll", self.base_url);
        let readings = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Vec<EnergyReading>>()
            .await?;

        Ok(readings)
    }
}
