use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[derive(Deserialize, Debug)]
struct ThermalZone {
    CurrentTemperature: u32,
}

fn get_cpu_temperature() -> Option<f32> {
    let com = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com.into()).ok()?;

    let results: Vec<ThermalZone> = wmi_con
        .raw_query("SELECT CurrentTemperature FROM MSAcpi_ThermalZoneTemperature")
        .ok()?;

    let raw = results.get(0)?.CurrentTemperature as f32;
    Some((raw / 10.0) - 273.15)
}

fn main() {
    match get_cpu_temperature() {
        Some(temp) => println!("CPU Temperature: {:.1} °C", temp),
        None => println!("Temperature not available via Windows API"),
    }
}
