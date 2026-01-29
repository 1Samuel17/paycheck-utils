//! Module for handling paycheck deductions.
//! Defines structures and functions for managing pre-tax and post-tax deductions.

/// Pre-tax deductions are applied before federal tax calculations.
pub enum PreTaxDeduction {
    Medical(Option<f32>),
    Dental(Option<f32>),
    Vision(Option<f32>),
    Traditional401K(Option<f32>),
    HSA(Option<f32>),
    FSA(Option<f32>),
}

/// Post-tax deductions are applied after federal tax calculations
pub enum PostTaxDeduction {
    Roth401K(Option<f32>),
    VoluntaryLife(Option<f32>),
    VoluntaryADD(Option<f32>),
    VoluntarySTD(Option<f32>),
    VoluntaryLTD(Option<f32>),
    WageGarnishment(Option<f32>), // e.g., child support, alimony
}

/// Struct to manage a collection of pre-tax deductions
pub struct PreTaxDeductions {
    pretax_deductions: Vec<PreTaxDeduction>,
}

/// Struct to manage a collection of post-tax deductions
pub struct PostTaxDeductions {
    posttax_deductions: Vec<PostTaxDeduction>,
}

impl PreTaxDeductions {
    /// Create a new collection of pre-tax deductions
    pub fn new(deductions: Vec<PreTaxDeduction>) -> Self {
        PreTaxDeductions {
            pretax_deductions: deductions,
        }
    }

    /// Add multiple pre-tax deductions to the collection
    pub fn add_pretax_deductions(&mut self, deductions: Vec<PreTaxDeduction>) {
        self.pretax_deductions.extend(deductions);
    }

    /// Get a reference to the list of pre-tax deductions
    pub fn get_pretax_deductions(&self) -> &Vec<PreTaxDeduction> {
        &self.pretax_deductions
    }

    /// Calculate the total amount of pre-tax deductions
    pub fn total_pretax_deductions(&self) -> f32 {
        self.pretax_deductions
            .iter()
            .fold(0.0, |acc, deduction| match deduction {
                PreTaxDeduction::Medical(amount)
                | PreTaxDeduction::Dental(amount)
                | PreTaxDeduction::Vision(amount)
                | PreTaxDeduction::Traditional401K(amount)
                | PreTaxDeduction::HSA(amount)
                | PreTaxDeduction::FSA(amount) => acc + amount.unwrap_or(0.0),
            })
    }
}

impl PostTaxDeductions {
    /// Create a new collection of post-tax deductions
    pub fn new(deductions: Vec<PostTaxDeduction>) -> Self {
        PostTaxDeductions {
            posttax_deductions: deductions,
        }
    }

    /// Add multiple post-tax deductions to the collection
    pub fn add_posttax_deductions(&mut self, deductions: Vec<PostTaxDeduction>) {
        self.posttax_deductions.extend(deductions);
    }

    /// Get a reference to the list of post-tax deductions
    pub fn get_posttax_deductions(&self) -> &Vec<PostTaxDeduction> {
        &self.posttax_deductions
    }

    /// Calculate the total amount of post-tax deductions
    pub fn total_posttax_deductions(&self) -> f32 {
        self.posttax_deductions
            .iter()
            .fold(0.0, |acc, deduction| match deduction {
                PostTaxDeduction::Roth401K(amount)
                | PostTaxDeduction::VoluntaryLife(amount)
                | PostTaxDeduction::VoluntaryADD(amount)
                | PostTaxDeduction::VoluntarySTD(amount)
                | PostTaxDeduction::VoluntaryLTD(amount)
                | PostTaxDeduction::WageGarnishment(amount) => acc + amount.unwrap_or(0.0),
            })
    }
}

// UNIT TESTS FOR DEDUCTIONS MODULE

#[cfg(test)]
mod tests {
    use super::*;
    use PostTaxDeduction::*;
    use PreTaxDeduction::*;
    #[test]
    fn test_total_pretax_deductions() {
        let deductions = PreTaxDeductions::new(vec![
            Medical(Some(150.0)),
            Dental(Some(50.0)),
            Traditional401K(Some(200.0)),
        ]);
        let total = deductions.total_pretax_deductions();
        assert_eq!(total, 400.0);
    }
    #[test]
    fn test_get_pretax_deductions() {
        let deductions = PreTaxDeductions::new(vec![
            Medical(Some(150.0)),
            Dental(Some(50.0)),
            Traditional401K(Some(200.0)),
        ]);
        let pretax_list = deductions.get_pretax_deductions();
        assert_eq!(pretax_list.len(), 3);
    }
    #[test]
    fn test_add_pretax_deductions() {
        let mut deductions = PreTaxDeductions::new(vec![Medical(Some(150.0))]);
        deductions.add_pretax_deductions(vec![Dental(Some(50.0)), Traditional401K(Some(200.0))]);
        let total = deductions.total_pretax_deductions();
        assert_eq!(total, 400.0);
    }
    #[test]
    fn test_total_posttax_deductions() {
        let deductions = PostTaxDeductions::new(vec![
            VoluntaryLife(Some(30.0)),
            VoluntarySTD(Some(22.0)),
            VoluntaryLTD(Some(34.0)),
            WageGarnishment(Some(600.0)),
        ]);
        let total = deductions.total_posttax_deductions();
        assert_eq!(total, 686.0);
    }
    #[test]
    fn test_get_posttax_deductions() {
        let deductions = PostTaxDeductions::new(vec![
            VoluntaryLife(Some(30.0)),
            VoluntarySTD(Some(22.0)),
            VoluntaryLTD(Some(34.0)),
            WageGarnishment(Some(600.0)),
        ]);
        let posttax_list = deductions.get_posttax_deductions();
        assert_eq!(posttax_list.len(), 4);
    }
    #[test]
    fn test_add_posttax_deductions() {
        let mut deductions = PostTaxDeductions::new(vec![VoluntaryLife(Some(30.0))]);
        deductions.add_posttax_deductions(vec![
            VoluntarySTD(Some(22.0)),
            VoluntaryLTD(Some(34.0)),
            WageGarnishment(Some(600.0)),
        ]);
        let total = deductions.total_posttax_deductions();
        assert_eq!(total, 686.0);
    }
}
