use crate::types::{ClientAttributeValue, DateTime};

#[derive(Debug)]
pub struct DateRange {
    pub id: String,
    pub class: Option<String>,
    pub start_date: DateTime,
    pub end_date: Option<DateTime>,
    pub duration: Option<f64>,
    pub planned_duration: Option<f64>,
    pub client_attributes: Vec<(String, ClientAttributeValue)>,
    pub end_on_next: bool,
}
