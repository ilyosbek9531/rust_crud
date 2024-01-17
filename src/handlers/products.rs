use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
use crate::{AppState, models::products::{ProductFilterOptions, ProductModel, CreateProduct, UpdateProduct}, schema::PathOptions};

#[get("")]
async fn get_products(data: web::Data<AppState>, opts: web::Query<ProductFilterOptions>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.offset.unwrap_or(1) - 1) * 10;
    let category_id = opts.category_id;

    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products WHERE category_id = COALESCE($1, category_id) ORDER BY created_at LIMIT $2 OFFSET $3",
        category_id,
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(products) => {
            let query_count = sqlx::query_scalar("SELECT COUNT(*) FROM products WHERE category_id = COALESCE($1, category_id)")
                .bind(category_id)
                .fetch_one(&data.db)
                .await;
            let product_count: i64 = match query_count {
                Ok(count) => count,
                Err(err) => {
                    let json_error = json!({
                        "status": "error",
                        "message": format!("{}", err)
                    });
                    return HttpResponse::InternalServerError().json(json_error);
                }
            };
            let json_response = json!({
                "status": "success",
                "count": product_count,
                "data": products
            });
            return HttpResponse::Ok().json(json_response)
        },
        Err(err) => {
            let json_error = json!({
                "status": "error",
                "message": format!("{}", err)
            });
            return HttpResponse::InternalServerError().json(json_error);
        }
    };
}


#[post("")]
async fn create_product(data: web::Data<AppState>, body: web::Json<CreateProduct>) -> impl Responder {
    let query_result = sqlx::query_as!(
        ProductModel,
        "INSERT INTO products (product_name, price, category_id) VALUES ($1, $2, $3) RETURNING *",
        body.product_name,
        body.price,
        body.category_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(product) => {
            let json_response = json!({
                "status": "success",
                "data": product
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
async fn get_product(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let product_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products WHERE id = $1",
        product_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(product) => {
            let json_response = json!({
                "status": "success",
                "data": product
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
async fn update_product(data: web::Data<AppState>, path: web::Path<PathOptions>, body: web::Json<UpdateProduct>) -> impl Responder {
    let product_id = path.into_inner().id;

    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products WHERE id = $1",
        product_id
    )
        .fetch_one(&data.db)
        .await;

    let product = match query_result {
        Ok(product) => product,
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
        ProductModel,
        "UPDATE products SET product_name = $1, price = $2, category_id = $3, updated_at = $4 WHERE id = $5 RETURNING *",
        body.product_name.to_owned().unwrap_or(product.product_name),
        body.price.to_owned().unwrap_or(product.price),
        body.category_id.to_owned().unwrap_or(product.category_id),
        now,
        product_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(product) => {
            let json_response = json!({
                "status": "success",
                "data": product
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
async fn delete_product(data: web::Data<AppState>, path: web::Path<PathOptions>) -> impl Responder {
    let product_id = path.into_inner().id;

    let query_result = sqlx::query!("DELETE FROM products WHERE id = $1", product_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if query_result == 0 {
        let json_error = json!({
            "status": "error",
            "message": format!("No results found with ID: {}", product_id)
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
    let scope = web::scope("/products")
        .service(get_products)
        .service(create_product)
        .service(get_product)
        .service(update_product)
        .service(delete_product);
    
    cfg.service(scope);
}