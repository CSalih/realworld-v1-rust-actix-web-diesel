use super::entities::{UpdateUser, User};
use super::{presenters::UserResponse, requests};
use crate::appv2::drivers::middlewares::auth;
use crate::appv2::drivers::middlewares::state::AppState;
use crate::utils::api::ApiResponse;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn signin(state: web::Data<AppState>, form: web::Json<requests::Signin>) -> ApiResponse {
    let res = state
        .di_container
        .user_usecase
        .signin(&form.user.email, &form.user.password)?;
    Ok(res)
}

pub async fn signup(state: web::Data<AppState>, form: web::Json<requests::Signup>) -> ApiResponse {
    let res = state.di_container.user_usecase.signup(
        &form.user.email,
        &form.user.username,
        &form.user.password,
    )?;
    Ok(res)
}

pub async fn me(state: web::Data<AppState>, req: HttpRequest) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let res = state.di_container.user_usecase.me(&current_user)?;
    Ok(res)
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<requests::Update>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let user = User::update(
        conn,
        current_user.id,
        UpdateUser {
            email: form.user.email.clone(),
            username: form.user.username.clone(),
            password: form.user.password.clone(),
            image: form.user.image.clone(),
            bio: form.user.bio.clone(),
        },
    )?;
    let token = &user.generate_token()?;
    let res = UserResponse::from((user, token.to_string()));
    Ok(HttpResponse::Ok().json(res))
}
