use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use components::{buttons::Button, buttons::Styled, page};
use maud::html;

mod components;

#[get("/")]
async fn index() -> impl Responder {
    page::page(html! {
        body {
            h1 ."text-8xl font-black" { "Actix HTML" }
            button hx-get="/clicked" .(Button::DEFAULT) { "HTMX" }
        }
    })
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
