#[derive(Debug)]
pub enum DUnit {
    M,
    KM,
    CM,
    MM,
    MI,
    YD,
    FT,
    IN,
}

impl DUnit {
    pub fn describe() -> &'static str {
        "{{\n\tDUnit::M => \"Meter\",\n\tDUnit::KM => \"Kilometer\",\n\tDUnit::CM => \"Centimeter\"\
        ,\n\tDUnit::MM => \"Millimeter\",\n\tDUnit::MI => \"Mile\",\n\tDUnit::YD => \"Yard\",\n\t\
        DUnit::FT => \"Foot\",\n\tDUnit::IN => \"Inch\"\n}}"
    }
}
