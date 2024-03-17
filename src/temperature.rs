#[derive(Debug)]
pub enum TUnit {
    C,
    F,
    K,
}

impl TUnit {
    pub fn describe() -> &'static str {
        "{{\n\tTUnit::C => \"Celsius\",\n\tTUnit::F => \"Fahrenheit\",\n\tTUnit::K => \"Kelvin\"\n}}"
    }
}
