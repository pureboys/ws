use std::sync::Mutex;
use crate::models::Course;

pub struct AppState {
    pub health_check_reponse: String,
    pub visit_count: Mutex<u32>,
    pub courses:Mutex<Vec<Course>>,
}
