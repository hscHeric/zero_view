use crate::api::google_sheets::EnergyReading;
use chrono::{DateTime, Utc};

const VOLTAGE: f64 = 220.0;

#[derive(Debug, Clone)]
pub struct EnergyMonitor {
    power_watts: f64,
    current_amperes: f64,
    timestamp: DateTime<Utc>,
}

impl EnergyMonitor {
    pub fn new(data: EnergyReading) -> Option<Self> {
        Some(Self {
            power_watts: VOLTAGE * data.current(),
            current_amperes: data.current(),
            timestamp: data.parse_timestamp()?,
        })
    }

    pub fn power_watts(&self) -> f64 {
        self.power_watts
    }

    pub fn current_amperes(&self) -> f64 {
        self.current_amperes
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}
