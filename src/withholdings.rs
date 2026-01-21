/// Module for calculating payroll taxes including federal, state, and local taxes.

pub enum Withholding {
    Federal(f32), // percentage
    SocialSecurity(f32), // percentage
    Medicare(f32), // percentage
}


