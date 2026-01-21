/// Module for handling payroll deductions other than taxes.


pub enum PayrollDeduction {
    Medical(Option<f32>), // health insurance (fixed amount)
    Dental(Option<f32>), // health insurance (fixed amount)
    Vision(Option<f32>), // health insurance (fixed amount)
    Retirement401k(Option<f32>), // retirement (percentage)
    RetirementRoth401k(Option<f32>), // retirement (percentage)
    HSA(Option<f32>), // health savings account (fixed amount)
    FSA(Option<f32>), // flexible spending account (fixed amount)
    VoluntaryADD(Option<f32>), // accidental death & dismemberment (fixed amount)
    VoluntaryLife(Option<f32>), // group life insurance (fixed amount)
    VoluntaryLTD(Option<f32>), // long term disability (fixed amount)
    VoluntarySTD(Option<f32>), // short term disability (fixed amount)
}


pub struct PayrollDeductions {
    pub deductions: Vec<PayrollDeduction>
}

impl PayrollDeductions {
    pub fn total_payroll_deductions(&self) -> f32 {
        self.deductions.iter().map(|deduction| {
            match deduction {
                PayrollDeduction::Medical(amount) |
                PayrollDeduction::Dental(amount) |
                PayrollDeduction::Vision(amount) |
                PayrollDeduction::HSA(amount) |
                PayrollDeduction::FSA(amount) |
                PayrollDeduction::VoluntaryADD(amount) |
                PayrollDeduction::VoluntaryLife(amount) |
                PayrollDeduction::VoluntaryLTD(amount) |
                PayrollDeduction::VoluntarySTD(amount) => amount.unwrap_or(0.0),
                PayrollDeduction::Retirement401k(percentage) |
                PayrollDeduction::RetirementRoth401k(percentage) => {
                    // For percentage-based deductions, we would need the gross income to calculate the actual amount.
                    // Here we return 0.0 as a placeholder.
                    0.0
                }
            }
        }).sum()
    }
}