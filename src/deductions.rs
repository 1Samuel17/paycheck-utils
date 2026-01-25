/// Module for handling payroll deductions other than taxes.


// standard payroll deduction categories per pay period (default 2 pay periods per month)
pub enum PaycheckDeduction {
    Medical(Option<f32>), // health insurance (fixed amount)
    Dental(Option<f32>), // health insurance (fixed amount)
    Vision(Option<f32>), // health insurance (fixed amount)
    Retirement401k(Option<f32>), // retirement (percentage) **PRETAX
    RetirementRoth401k(Option<f32>), // retirement (percentage)
    HSA(Option<f32>), // health savings account (fixed amount) **PRETAX
    FSA(Option<f32>), // flexible spending account (fixed amount)
    VoluntaryADD(Option<f32>), // accidental death & dismemberment (fixed amount)
    VoluntaryLife(Option<f32>), // group life insurance (fixed amount)
    VoluntaryLTD(Option<f32>), // long term disability (fixed amount)
    VoluntarySTD(Option<f32>), // short term disability (fixed amount)
}


pub struct PaycheckDeductions {
    pub deductions: Vec<PaycheckDeduction>,
}

impl PaycheckDeductions {
    pub fn total_paycheck_deductions(&self) -> f32 {
        self.deductions.iter().map(|deduction| {
            match deduction {
                PaycheckDeduction::Medical(amount) |
                PaycheckDeduction::Dental(amount) |
                PaycheckDeduction::Vision(amount) |
                PaycheckDeduction::HSA(amount) |
                PaycheckDeduction::FSA(amount) |
                PaycheckDeduction::VoluntaryADD(amount) |
                PaycheckDeduction::VoluntaryLife(amount) |
                PaycheckDeduction::VoluntaryLTD(amount) |
                PaycheckDeduction::VoluntarySTD(amount) => amount.unwrap_or(0.0),
                PaycheckDeduction::Retirement401k(percentage) |
                PaycheckDeduction::RetirementRoth401k(percentage) => {
                    // For percentage-based deductions, we would need the gross income to calculate the actual amount.
                    // Here we return 0.0 as a placeholder.
                    0.0
                }
            }
        }).sum()
    }
}