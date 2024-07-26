use actix_web::web;
use crate::handler::{get_course_detail, get_courses_for_teacher, health_check_handler, new_course};

pub fn general_routers(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check_handler));
}

pub fn course_routers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses").
        route("/", web::post().to(new_course)).
        route("/{user_id}", web::get().to(get_courses_for_teacher)).
        route("/{user_id}/{course_id}", web::get().to(get_course_detail))
    );
}
