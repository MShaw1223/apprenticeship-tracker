#[derive(Debug, Clone)]
pub struct Apprenticeship {
    pub id: i64,
    pub area: String,
    pub close_date: String,
    pub company: String,
    pub date_applied: String,
    pub level: i8,
    pub notes: String,
    pub pay: f64,
    pub requirements: String,
    pub role: String,
    pub stage: String,
}

// A listing as typed into the entry form, before level and pay are validated.
#[derive(Debug, Clone, Default)]
pub struct ApprenticeshipInput {
    pub area: String,
    pub close_date: String,
    pub company: String,
    pub date_applied: String,
    pub level: String,
    pub notes: String,
    pub pay: String,
    pub requirements: String,
    pub role: String,
    pub stage: String,
}
