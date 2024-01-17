use actix_web::{get, post, patch, delete, web, Responder, HttpResponse};
use serde_json::json;
use chrono::Utc;

use crate::{AppState, schema::{FilterOptions, PathOptions}, models::users::{UserModel, CreateUser, UpdateUser}};

#[get("")]
async fn get_users(data: web::Data<AppState>, opts: web::Query<FilterOptions>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.offset.unwrap_or(1) - 1) * 1;

    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(users) => {
            let query_count = sqlx::query_scalar("SELECT COUNT(*) FROM users").fetch_one(&data.db).await;

            let users_count: i64 = match query_count {
                Ok(count) => count,
                Err(err) => {
                   let json_error = json!({
                        "status": "error",
                        "message": format!("{}", err)
                    }); 
                    return HttpResponse::InternalServerError().json(json_error)
                }
            };
            let json_response = json!({
                "status": "success",
                "count": users_count,
                "data": users
            });

            return HttpResponse::Ok().json(json_response)
        },
        Err(err) => {
            let json_error = json!({
                "status": "error",
                "message": format!("{}", err)
            });
            return HttpResponse::InternalServerError().json(json_error)
        }
    }
}

#[post("")]
async fn create_user(data: web::Data<AppState>, body: web::Json<CreateUser>) -> impl Responder {
    let query_result = sqlx::query_as!(
        UserModel,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING *",
        body.username.to_string(),
        body.email.to_owned().unwrap_or("".to_string())
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(user) => {
            let json_response = json!({
                "status": "success",
                "data": user
            });

            return HttpResponse::Ok().json(json_response);
        },
        Err(err) => {
            let json_error = json!({
                "status": "error",
                "message": format!("{}", err)
            });

            return HttpResponse::InternalServerError().json(json_error);
        }
    }
}

#[get("/{id}")]
async fn get_user(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let user_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(user) => {
            let json_response = json!({
                "status": "succcess",
                "data": user
            });
            return HttpResponse::Ok().json(json_response);
        },
        Err(err) => {
            let json_error = json!({
                "status": "error",
                "message": format!("{}", err)
            });

            return HttpResponse::InternalServerError().json(json_error);
        }
    }
}


#[patch("/{id}")]
async fn update_user(data: web::Data<AppState>, path: web::Path<PathOptions>, body: web::Json<UpdateUser>) -> impl Responder {
    let user_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
        .fetch_one(&data.db)
        .await;

    let user = match query_result {
        Ok(user) => user,
        Err(err) => {
            let json_error = json!({
                "status": "error",
                "message": format!("{}", err)
            });

            return HttpResponse::InternalServerError().json(json_error);
        }
    };

    let now = Utc::now();

    let query_result = sqlx::query_as!(
        UserModel,
        "UPDATE users SET username = $1, email = $2, updated_at = $3 WHERE id = $4 RETURNING *",
        body.username.to_owned().unwrap_or(user.username),
        body.email.to_owned().unwrap_or(user.email.unwrap_or("".to_string())),
        now,
        user_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(user) => {
            let json_response = json!({
                "status": "success",
                "data": user
            });

            return HttpResponse::Ok().json(json_response);
        },
        Err(err) => {
            let json_error = json!({
                "status": "error",
                "message": format!("{}", err)
            });
            return HttpResponse::InternalServerError().json(json_error);
        }
    }
}


#[delete("/{id}")]
async fn delete_user(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let user_id = path.into_inner().id;

    let query_result = sqlx::query_scalar!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if query_result == 0 {
        let json_error = json!({
            "status": "not found",
            "message": format!("There is no user with ID: {}", user_id)
        });

        return HttpResponse::NotFound().json(json_error);
    }

    let json_response = json!({
        "status": "success",
        "message": format!("User removed with ID: {}", user_id)
    });

    HttpResponse::Ok().json(json_response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(get_users)
        .service(create_user)
        .service(get_user)
        .service(update_user)
        .service(delete_user);
    cfg.service(scope);
}