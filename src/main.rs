use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use askama::Template;
use askama_actix::TemplateToResponse;
use maud::html;

mod components;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    iter: Vec<i32>,
}

#[get("/")]
async fn index() -> impl Responder {
    let index = IndexTemplate {
        iter: (0..10).collect(),
    };
    index.to_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(
                Files::new("/css", "./dist")
                    .use_last_modified(true)
                    .use_etag(true),
            )
            .service(
                Files::new("/js", "./node_modules")
                    .use_last_modified(true)
                    .use_etag(true),
            )
            .route(
                "/clicked",
                web::get().to(|| async {
                    html! {
                        "Clicked!"
                    }
                }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
