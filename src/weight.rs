#[derive(Debug)]
pub enum WUnit {
    MT,
    TN,
    KG,
    LB,
    GM,
    OZ,
}

impl WUnit {
    pub fn describe() -> &'static str {
        "{{\n\tWUnit::MT => \"Metric Ton\",\n\tWUnit::TN => \"Tonne\",\n\tWUnit::KG => \"Kilogram\"\
        ,\n\tWUnit::LB => \"Pound\",\n\tWUnit::GM => \"Gram\",\n\tWUnit::OZ => \"Ounce\"\n}}"
    }
}
