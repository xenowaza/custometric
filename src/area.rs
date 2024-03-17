#[derive(Debug)]
pub enum AUnit {
    SQM,
    SQKM,
    SQCM,
    SQMM,
    SQMI,
    SQYD,
    SQFT,
    SQIN,
}

impl AUnit {
    pub fn describe() -> &'static str {
        "{{\n\tAUnit::SQM => \"Square Meter\",\n\tAUnit::SQKM => \"Square Kilometer\",\n\tAUnit::\
        SQCM => \"Square Centimeter\",\n\tAUnit::SQMM => \"Square Millimeter\",\n\tAUnit::SQMI => \
        \"Square Mile\",\n\tAUnit::SQYD => \"Square Yard\",\n\tAUnit::SQFT => \"Square Foot\",\n\t\
        AUnit::SQIN => \"Square Inch\"\n}}"
    }
}
