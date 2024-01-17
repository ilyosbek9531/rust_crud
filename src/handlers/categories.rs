use actix_web::{get, post, patch, delete, web, Responder, HttpResponse};
use serde_json::json;
use chrono::Utc;

use crate::{AppState, schema::{FilterOptions, PathOptions}, models::categories::{CategoryModel, CreateCategory, UpdateCategory}};

#[get("")]
async fn get_categories(data: web::Data<AppState>, opts: web::Query<FilterOptions>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.offset.unwrap_or(1) - 1) * 1;

    let query_result = sqlx::query_as!(
        CategoryModel,
        "SELECT * FROM categories ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(categories) => {
            let query_count = sqlx::query_scalar("SELECT COUNT(*) FROM categories").fetch_one(&data.db).await;

            let categories_count: i64 = match query_count {
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
                "count": categories_count,
                "data": categories
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
async fn create_category(data: web::Data<AppState>, body: web::Json<CreateCategory>) -> impl Responder {
    let query_result = sqlx::query_as!(
        CategoryModel,
        "INSERT INTO categories (category_name) VALUES ($1) RETURNING *",
        body.category_name.to_string()
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(category) => {
            let json_response = json!({
                "status": "success",
                "data": category
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
async fn get_category(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let category_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        CategoryModel,
        "SELECT * FROM categories WHERE id = $1",
        category_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(category) => {
            let json_response = json!({
                "status": "succcess",
                "data": category
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
async fn update_category(data: web::Data<AppState>, path: web::Path<PathOptions>, body: web::Json<UpdateCategory>) -> impl Responder {
    let category_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        CategoryModel,
        "SELECT * FROM categories WHERE id = $1",
        category_id
    )
        .fetch_one(&data.db)
        .await;

    let category = match query_result {
        Ok(category) => category,
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
        CategoryModel,
        "UPDATE categories SET category_name = $1, updated_at = $2 WHERE id = $3 RETURNING *",
        body.category_name.to_owned().unwrap_or(category.category_name),
        now,
        category_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(category) => {
            let json_response = json!({
                "status": "success",
                "data": category
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
async fn delete_category(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let category_id = path.into_inner().id;

    let query_result = sqlx::query_scalar!("DELETE FROM categories WHERE id = $1", category_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if query_result == 0 {
        let json_error = json!({
            "status": "not found",
            "message": format!("There is no category with ID: {}", category_id)
        });

        return HttpResponse::NotFound().json(json_error);
    }

    let json_response = json!({
        "status": "success",
        "message": format!("Category removed with ID: {}", category_id)
    });

    HttpResponse::Ok().json(json_response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/categories")
        .service(get_categories)
        .service(create_category)
        .service(get_category)
        .service(update_category)
        .service(delete_category);
    cfg.service(scope);
}