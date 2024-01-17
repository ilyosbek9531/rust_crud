use actix_web::{get, post, patch, delete, HttpResponse, Responder, web};
use chrono::Utc;
use serde_json::json;

use crate::{AppState, models::purchases::{PurchaseFilterOptions, PurchaseModel, CreatePurchase, UpdatePurchase}, schema::PathOptions};

#[get("")]
async fn get_purchases(data: web::Data<AppState>, opts: web::Query<PurchaseFilterOptions>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.offset.unwrap_or(1) - 1) * limit;
    let product_id = opts.product_id;
    let user_id = opts.user_id;

    let query_result = sqlx::query_as!(
        PurchaseModel,
        "SELECT * FROM purchases WHERE product_id = COALESCE($1, product_id) AND user_id = COALESCE($2, user_id) ORDER BY created_at LIMIT $3 OFFSET $4",
        product_id,
        user_id,
        limit as i32, 
        offset as i32
    )
        .fetch_all(&data.db)
        .await;


    match query_result {
        Ok(purchases) => {
            let query_count = sqlx::query_scalar("SELECT COUNT(*) FROM purchases WHERE product_id = COALESCE($1, product_id) AND user_id = COALESCE($2, user_id)")
                .bind(product_id)
                .bind(user_id)
                .fetch_one(&data.db)
                .await;

            let purchase_count: i64 = match query_count {
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
                "count": purchase_count,
                "data": purchases
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
async fn create_purchase(data: web::Data<AppState>, body: web::Json<CreatePurchase>) -> impl Responder {
    let query_result = sqlx::query_as!(
        PurchaseModel,
        "INSERT INTO purchases (product_id, user_id) VALUES ($1, $2) RETURNING *",
        body.product_id,
        body.user_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(purchase) => {
            let json_response = json!({
                "status": "success",
                "data": purchase
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
async fn get_purchase(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let purchase_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        PurchaseModel,
        "SELECT * FROM purchases WHERE id = $1",
        purchase_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(purchase) => {
            let json_response = json!({
                "status": "success",
                "data": purchase
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
async fn update_purchase(data: web::Data<AppState>, path: web::Path<PathOptions>, body: web::Json<UpdatePurchase>) -> impl Responder {
    let purchase_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        PurchaseModel,
        "SELECT * FROM purchases WHERE id = $1",
        purchase_id
    )
        .fetch_one(&data.db)
        .await;

    let purchase = match query_result {
        Ok(purchase) => purchase,
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
        PurchaseModel,
        "UPDATE purchases SET product_id = $1, user_id = $2, updated_at = $3 WHERE id = $4 RETURNING *",
        body.product_id.to_owned().unwrap_or(purchase.product_id),
        body.user_id.to_owned().unwrap_or(purchase.user_id),
        now,
        purchase_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(purchase) => {
            let json_response = json!({
                "status": "success",
                "data": purchase
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
async fn delete_purchase(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let purchase_id = path.into_inner().id;

    let query_result = sqlx::query!("DELETE FROM purchases WHERE id = $1", purchase_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if query_result == 0 {
        let json_error = json!({
            "status": "error",
            "message": format!("No results found with ID: {}", purchase_id)
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
    let scope = web::scope("/purchases")
        .service(get_purchases)
        .service(create_purchase)
        .service(get_purchase)
        .service(update_purchase)
        .service(delete_purchase);
    cfg.service(scope);
}