use actix_web::web;
use crate::handlers::course::{delete_course, get_course_detail, get_courses_for_teacher, post_new_course, update_course_detail};
use crate::handlers::general::health_check_handler;

pub fn general_routers(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check_handler));
}

pub fn course_routers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses")
        .route("/", web::post().to(post_new_course))
        .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
        .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
        .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
        .route("/{teacher_id}/{course_id}", web::put().to(update_course_detail))
    );
}
