#[derive(Debug)]
pub enum CUnit {
    ML,
    L,
    KL,
    C,
    PT,
    QT,
    GAL,
}

impl CUnit {
    pub fn describe() -> &'static str {
        "{{\n\tCUnit::ML => \"Milliliter\",\n\tCUnit::L => \"Liter\",\n\tCUnit::KL => \"Kiloliter\"\
        ,\n\tCUnit::C => \"Cup\",\n\tCUnit::PT => \"Pint\",\n\tCUnit::QT => \"Quart\",\n\tCUnit::\
        GAL => \"Gallon\"\n}}"
    }
}
