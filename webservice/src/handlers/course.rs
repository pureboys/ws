use actix_web::{HttpResponse, web};
use crate::db_access::course::{delete_course_db, get_course_detail_db, get_course_for_teacher_db, post_new_course_db, update_course_details_db};
use crate::errors::MyError;
use crate::models::course::{CreateCourse, UpdateCourse};
use crate::state::AppState;


pub async fn post_new_course(new_course: web::Json<CreateCourse>, app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    println!("Receive new course");
    post_new_course_db(&app_state.db, new_course.try_into()?).await
        .map(|course| HttpResponse::Ok().json(course))
}


pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, ) = path.into_inner();
    get_course_for_teacher_db(&app_state.db, teacher_id).await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = path.into_inner();
    get_course_detail_db(&app_state.db, teacher_id, course_id).await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = path.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id).await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_detail(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = path.into_inner();
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into()).await
        .map(|course| HttpResponse::Ok().json(course))
}


#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use actix_web::http::StatusCode;
    use actix_web::{ResponseError, web};
    use crate::handlers::course::{delete_course, get_course_detail, get_courses_for_teacher, post_new_course, update_course_detail};
    use crate::state::AppState;
    use crate::models::course::{CreateCourse, UpdateCourse};
    use std::sync::Mutex;

    // #[ignore]
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
        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test Course".into(),
            description: Some("Test Description".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });
        let resp = post_new_course(course, app_state).await.unwrap();
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
        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
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
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = sqlx::PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_reponse: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let resp = get_course_detail(app_state, params).await;
        match resp {
            Ok(_) => println!("Someting wrong ..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND)
        }
    }

    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = sqlx::PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_reponse: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = sqlx::PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_reponse: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 101));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("Something wrong ..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND)
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = sqlx::PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_reponse: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let update_course = UpdateCourse {
            name: Some("Updated Course".into()),
            description: Some("Updated Description".into()),
            format: None,
            level: Some("Intermediate".into()),
            price: None,
            duration: None,
            language: Some("Chinese".into()),
            structure: None,
        };
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let update_params = web::Json(update_course);
        let resp = update_course_detail(app_state, update_params, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
