pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    // TODO: Implement the function here
    match (from_unit, to_unit) {
        ("C", "C") | ("F", "F") | ("K", "K") => Ok(value),
        ("C", "F") => Ok(value * (9.0 / 5.0) + 32.0),
        ("C", "K") => Ok(value + 273.15),
        ("F", "C") => Ok((value - 32.0) * (5.0 / 9.0)),
        ("F", "K") => Ok((value - 32.0) * (5.0 / 9.0) + 273.15),
        ("K", "C") => Ok(value - 273.15),
        ("K", "F") => Ok((value - 273.15) * (9.0 / 5.0) + 32.0),
        (_, _) => Err(String::from("Invalid unit")),
    }
}

