#[derive(Debug, Clone)]
pub struct Apprenticeship {
    pub id: i32,
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
