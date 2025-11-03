use super::Battery;
use super::Dev;
use crate::devs::SensorReading;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoilSensor {
    name: Option<String>,
    device_sn: u64,
    soil_moisture: Option<SensorReading>,
    temp: Option<SensorReading>,
    humidity: Option<SensorReading>,
    light: Option<SensorReading>,
    battery: Option<SensorReading>,
    uptime: Option<u32>,
    last_active: DateTime<Utc>,
    fwver_value: Option<u32>,
    fwver_name: Option<String>,
    #[serde(default)]
    status: crate::devs::hb::DevStatus,
}

impl SoilSensor {
    pub fn new(
        id: u64,
        name: Option<String>,
        last_active: DateTime<Utc>,
        fwver_value: Option<u32>,
        fwver_name: Option<String>,
        status: crate::devs::hb::DevStatus,
        uptime: Option<u32>,
    ) -> Self {
        Self {
            name,
            device_sn: id,
            soil_moisture: None,
            temp: None,
            humidity: None,
            light: None,
            battery: None,
            uptime,
            last_active,
            fwver_value,
            fwver_name,
            status,
        }
    }
    pub fn set_soil_moisture(&mut self, soil_moisture: SensorReading) {
        self.soil_moisture = Some(soil_moisture);
    }
    pub fn set_temp(&mut self, temp: SensorReading) {
        self.temp = Some(temp);
    }
    pub fn set_humidity(&mut self, humidity: SensorReading) {
        self.humidity = Some(humidity);
    }
    pub fn set_light(&mut self, light: SensorReading) {
        self.light = Some(light);
    }
    pub fn set_battery(&mut self, battery: SensorReading) {
        self.battery = Some(battery);
    }
    pub fn temp(&self) -> Option<f32> {
        Some(self.temp?.to_float())
    }
    pub fn humidity(&self) -> Option<f32> {
        Some(self.humidity?.to_float())
    }
    pub fn light(&self) -> Option<f32> {
        Some(self.light?.to_float())
    }
    pub fn soil_moisture(&self) -> Option<f32> {
        Some(self.soil_moisture?.to_float())
    }
    pub fn battery(&self) -> Option<f32> {
        Some(self.battery?.to_float())
    }
    pub fn fwver(&self) -> Option<u32> {
        self.fwver_value
    }
    pub fn fwver_name(&self) -> Option<&str> {
        self.fwver_name.as_deref()
    }
    pub fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }
}
impl Battery for SoilSensor {
    fn battery(&self) -> Option<f32> {
        Some(self.battery?.to_float())
    }
    fn bat_pct(&self) -> Option<f32> {
        self.battery()
            .map(|b| (((b - 2.5) / (3.3 - 2.5)) * 100.0).clamp(0.0, 100.0))
    }
}
impl Dev for SoilSensor {
    fn dev_id(&self) -> String {
        format!("{:#08x}", self.device_sn)
    }
    fn dev_sn(&self) -> u64 {
        self.device_sn
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    fn last_active(&self) -> DateTime<Utc> {
        self.last_active
    }
    fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }
    fn display_name(&self) -> String {
        match self.name() {
            Some(name) => name.to_string(),
            None => self.dev_id(),
        }
    }
    fn dev_type(&self) -> &'static str {
        "Soil Sensor"
    }
    fn fwver(&self) -> Option<[u8; 4]> {
        self.fwver_value.map(|v| v.to_be_bytes())
    }
    fn fwver_name(&self) -> Option<String> {
        self.fwver_name.clone()
    }
    fn status(&self) -> crate::devs::hb::DevStatus {
        self.status
    }
    fn has_sensors(&self) -> bool {
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::devs::SensorReading;

    #[test]
    fn test_soil_sensor_new() {
        let sensor = SoilSensor {
            device_sn: 0x12345678,
            name: None,
            soil_moisture: None,
            temp: None,
            humidity: None,
            light: None,
            battery: Some(SensorReading { h: 2, l: 400000 }),
            uptime: None,
            last_active: Utc::now(),
            fwver_value: None,
            fwver_name: None,
            status: crate::devs::hb::DevStatus::RunningOk,
        };
        assert_eq!(sensor.dev_id(), "0x12345678");
        assert_eq!(sensor.bat_pct(), Some(0.0));
        assert_eq!(sensor.battery(), Some(2.4));
    }

    #[test]
    fn test_soil_sensor_temp() {
        let mut sensor = SoilSensor::new(
            0x12345678,
            None,
            Utc::now(),
            None,
            None,
            crate::devs::hb::DevStatus::RunningOk,
            None,
        );
        sensor.set_temp(SensorReading { h: 25, l: 500000 });
        assert_eq!(sensor.temp(), Some(25.5));
    }
}
