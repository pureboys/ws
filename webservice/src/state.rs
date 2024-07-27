use std::sync::Mutex;
use sqlx::PgPool;

pub struct AppState {
    pub health_check_reponse: String,
    pub visit_count: Mutex<u32>,
    // pub courses:Mutex<Vec<Course>>,
    pub db: PgPool
}
