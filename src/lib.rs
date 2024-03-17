pub mod area;
pub mod capacity;
pub mod distance;
pub mod temperature;
pub mod weight;

#[cfg(test)]
mod tests {
    use super::*;
    use area::AUnit;
    use capacity::CUnit;
    use distance::DUnit;
    use temperature::TUnit;
    use weight::WUnit;

    #[test]
    fn test_enums_provide_variants() {
        println!("{:?}", TUnit::C);
        println!("{:?}", DUnit::CM);
        println!("{:?}", CUnit::C);
        println!("{:?}", WUnit::GM);
        println!("{:?}", AUnit::SQCM);
    }
}
