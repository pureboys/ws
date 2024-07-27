use actix_web::{HttpResponse, web};
use crate::db_access::{get_course_detail_db, get_course_for_teacher_db, post_new_course_db};
use crate::models::Course;
use crate::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_reponse;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(new_course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {
    println!("Receive new course");
    // let course_count = app_state.courses.lock().unwrap().clone().into_iter()
    //     .filter(|course| course.teacher_id == new_course.teacher_id).collect::<Vec<Course>>().len();
    //
    // let new_course = Course {
    //     teacher_id: new_course.teacher_id,
    //     id: Some(course_count + 1),
    //     name: new_course.name.clone(),
    //     time: Some(Utc::now().naive_utc()),
    // };
    // app_state.courses.lock().unwrap().push(new_course);
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}


pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    // let teacher_id: usize = params.into_inner();
    // let filtered_courses = app_state.courses.lock().unwrap().clone().into_iter()
    //     .filter(|course| course.teacher_id == teacher_id).collect::<Vec<Course>>();
    //
    // if filtered_courses.len() > 0 {
    //     HttpResponse::Ok().json(filtered_courses)
    // } else {
    //     HttpResponse::Ok().json("No courses found for teacher".to_string())
    // }
    let (teacher_id,) = params.into_inner();
    let courses = get_course_for_teacher_db(&app_state.db, teacher_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    // let (teacher_id, course_id) = params.into_inner();
    // let selected_courses = app_state.courses.lock().unwrap().clone().into_iter()
    //     .find(|x| x.teacher_id == teacher_id && x.id == Some(course_id)).ok_or("Course not found");
    //
    // if let Ok(course) = selected_courses {
    //     HttpResponse::Ok().json(course)
    // } else {
    //     HttpResponse::Ok().json("Course not found".to_string())
    // }
    let (teacher_id, course_id) = params.into_inner();
    let course = get_course_detail_db(&app_state.db, teacher_id, course_id).await;
    HttpResponse::Ok().json(course)
}


#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use crate::handler::{get_course_detail, get_courses_for_teacher, new_course};
    use crate::state::AppState;
    use crate::models::Course;
    use std::sync::Mutex;


    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = sqlx::PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_reponse: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let course = web::Json(Course {
            id: Some(3),
            teacher_id: 1,
            name: "Test course".into(),
            time: None,
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = sqlx::PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_reponse: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_courses_for_teacher(app_state, teacher_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = sqlx::PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_reponse: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
