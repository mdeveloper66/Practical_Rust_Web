#[macro_use]
extern crate diesel;

use crate::schema::cats::dsl::*;
use actix_files::Files;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use errors::UserError;
use models::*;
use std::env;
use validator::Validate;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod errors;
mod models;
mod schema;

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/cats", web::get().to(cats_endpoint))
            .route("/cat/{id}", web::get().to(cat_endpoint)),
    );
}

fn setup_database() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Can't get database url");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Faild to connect to database")
}

async fn cats_endpoint(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Can't get db connection from pool");
    let cats_data = web::block(move || cats.limit(100).load::<Cat>(&connection))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    return Ok(HttpResponse::Ok().json(cats_data));
}

async fn cat_endpoint(
    pool: web::Data<DbPool>,
    cat_id: web::Path<CatEndpointPath>,
) -> Result<HttpResponse, UserError> {
    cat_id.validate().map_err(|_| UserError::ValidateError)?;
    let connection = pool.get().map_err(|_| UserError::DBPoolGetError)?;
    let cat_data = web::block(move || cats.filter(id.eq(cat_id.id)).first::<Cat>(&connection))
        .await
        .map_err(|e| match e {
            error::BlockingError::Error(diesel::result::Error::NotFound) => {
                UserError::NotFoundError
            }
            _ => UserError::UnexpectedError,
        })?;
    Ok(HttpResponse::Ok().json(cat_data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listen on port 8080");
    env_logger::init();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
    .unwrap();
    builder
    .set_private_key_file("key-no-password.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    let pool = setup_database();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .configure(api_config)
            .service(Files::new("/", "static").show_files_listing())
    })
   // .bind("127.0.0.1:8080")?
   .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_cats_endpoint_get() {
        let pool = setup_database();
        let mut app = test::init_service(App::new().data(pool.clone()).configure(api_config)).await;
        let req = test::TestRequest::get().uri("/api/cats").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
