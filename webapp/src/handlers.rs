use actix_web::{HttpResponse, web};
use actix_web::Error;
use serde_json::json;
use crate::wa::errors::MyError;
use crate::wa::models::{TeacherRegisterForm, TeacherResponse};

pub async fn get_all_teachers(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let awc_clinet = awc::Client::default();

    let res = awc_clinet.get("http://localhost:3000/teachers/")
        .send()
        .await
        .unwrap()
        .json::<Vec<TeacherResponse>>()
        .await
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("teachers", &res);

    let s = tmpl.render("teachers.html", &ctx).map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}


pub async fn show_register_form(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");

    let s = tmpl.render("register.html", &ctx).map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<TeacherRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;

    if params.name == "Dave" {
        ctx.insert("error", "Dave is already registered");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl.render("register.html", &ctx).map_err(|_| MyError::TeraError("Template error".to_string()))?;
    } else {
        let new_teacher = json!({
            "name": &params.name,
            "picture_url": &params.imageurl,
            "profile": &params.profile
        });

        let awc_client = awc::Client::default();
        let res = awc_client.post("http://localhost:3000/teachers/")
            .send_json(&new_teacher)
            .await
            .unwrap()
            .body()
            .await?;
        let res_str = std::str::from_utf8(&res)?;
        let teacher_response: TeacherResponse = serde_json::from_str(res_str)?;

        s = format!("Congratulations! Your id is:{}", teacher_response.id);
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
