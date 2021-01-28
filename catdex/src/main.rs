use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde_json::json;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Catdex",
        "cats": [
        {
            "name": "British short hair",
            "image_path":
                "/static/images/british-short-hair.jpg"
        },
        {
            "name": "Persian",
            "image_path": "/static/images/persian.jpg"
        },
        {
            "name": "Ragdoll",
            "image_path": "/static/images/ragdoll.jpg"
        }
        ]
    });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening to port 8080");
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
