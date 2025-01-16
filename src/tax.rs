#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxGroup {
    Food,
    Alcohol,
    NonTaxable,
    Other,
}

impl TaxGroup {
    pub const ALL: [TaxGroup; 4] = [
        TaxGroup::Food,
        TaxGroup::Alcohol,
        TaxGroup::NonTaxable,
        TaxGroup::Other,
    ];

    pub fn tax_rate(&self) -> f32 {
        match self {
            TaxGroup::Food => 0.08,
            TaxGroup::Alcohol => 0.10,
            TaxGroup::NonTaxable => 0.0,
            TaxGroup::Other => 0.08,
        }
    }
}

impl std::fmt::Display for TaxGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TaxGroup::Food => "Food (8%)",
                TaxGroup::Alcohol => "Alcohol (10%)",
                TaxGroup::NonTaxable => "Non-taxable",
                TaxGroup::Other => "Other (8%)",
            }
        )
    }
}
