use actix_web::{get, post, patch, delete, HttpResponse, Responder, web};
use chrono::Utc;
use serde_json::json;

use crate::{AppState, models::ratings::{RatingFilterOptions, RatingModel, CreateRating, UpdateRating}, schema::PathOptions};

#[get("")]
async fn get_ratings(data: web::Data<AppState>, opts: web::Query<RatingFilterOptions>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.offset.unwrap_or(1) - 1) * limit;
    let product_id = opts.product_id;
    let user_id = opts.user_id;

    let query_result = sqlx::query_as!(
        RatingModel,
        "SELECT * FROM ratings WHERE product_id = COALESCE($1, product_id) AND user_id = COALESCE($2, user_id) ORDER BY created_at LIMIT $3 OFFSET $4",
        product_id,
        user_id,
        limit as i32, 
        offset as i32
    )
        .fetch_all(&data.db)
        .await;


    match query_result {
        Ok(ratings) => {
            let query_count = sqlx::query_scalar("SELECT COUNT(*) FROM ratings WHERE product_id = COALESCE($1, product_id) AND user_id = COALESCE($2, user_id)")
                .bind(product_id)
                .bind(user_id)
                .fetch_one(&data.db)
                .await;

            let rating_count: i64 = match query_count {
                Ok(count) => count,
                Err(err) => {
                    let json_error = json!({
                        "status": "error",
                        "data": format!("{}", err)
                    });
                    return HttpResponse::InternalServerError().json(json_error);
                }
            };
            let json_response = json!({
                "status": "success",
                "count": rating_count,
                "data": ratings
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


#[post("")]
async fn create_rating(data: web::Data<AppState>, body: web::Json<CreateRating>) -> impl Responder {
    let query_result = sqlx::query_as!(
        RatingModel,
        "INSERT INTO ratings (rating, product_id, user_id) VALUES ($1, $2, $3) RETURNING *",
        body.rating,
        body.product_id,
        body.user_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(rating) => {
            let json_response = json!({
                "status": "success",
                "data": rating
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
async fn get_rating(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let rating_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        RatingModel,
        "SELECT * FROM ratings WHERE id = $1",
        rating_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(rating) => {
            let json_response = json!({
                "status": "success",
                "data": rating
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
async fn update_rating(data: web::Data<AppState>, path: web::Path<PathOptions>, body: web::Json<UpdateRating>) -> impl Responder {
    let rating_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        RatingModel,
        "SELECT * FROM ratings WHERE id = $1",
        rating_id
    )
        .fetch_one(&data.db)
        .await;

    let rating = match query_result {
        Ok(rating) => rating,
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
        RatingModel,
        "UPDATE ratings SET rating = $1, product_id = $2, user_id = $3, updated_at = $4 WHERE id = $5 RETURNING *",
        body.rating.to_owned().unwrap_or(rating.rating),
        body.product_id.to_owned().unwrap_or(rating.product_id),
        body.user_id.to_owned().unwrap_or(rating.user_id),
        now,
        rating_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(rating) => {
            let json_response = json!({
                "status": "success",
                "data": rating
            });
            return HttpResponse::Ok().json(json_response);
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

#[delete("/{id}")]
async fn delete_rating(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let rating_id = path.into_inner().id;

    let query_result = sqlx::query!("DELETE FROM ratings WHERE id = $1", rating_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if query_result == 0 {
        let json_error = json!({
            "status": "error",
            "message": format!("No results found with ID: {}", rating_id)
        });
        return HttpResponse::InternalServerError().json(json_error)
    };

    let json_response = json!({
        "status": "success",
        "message": "successfully deleted"
    });

    HttpResponse::Ok().json(json_response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/ratings")
        .service(get_ratings)
        .service(create_rating)
        .service(get_rating)
        .service(update_rating)
        .service(delete_rating);
    cfg.service(scope);
}