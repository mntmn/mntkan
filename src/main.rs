extern crate actix_web;
extern crate actix_files;
extern crate dotenv;
extern crate toml;
#[macro_use]
extern crate diesel;

use diesel::sqlite::SqliteConnection;

use actix_web::{error, get, middleware, post, web, App, Error, FromRequest, HttpRequest, HttpResponse, HttpServer};
use actix_files::Files;
use diesel::r2d2::{self, ConnectionManager};

use tera::Tera;

mod models;
mod schema;
mod routes;
mod mntconfig;

use crate::routes::cards::*;
//use crate::models::Card;
use crate::mntconfig::Config;

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    let config_str = std::fs::read_to_string("mntconfig.toml").unwrap();
    let mntconfig:Config = toml::from_str(&config_str).unwrap();

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    // FIXME: why move?
    HttpServer::new(move || {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        
        App::new()
            // tera templating
            .data(tera)
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .data(mntconfig.clone())
            .wrap(middleware::Logger::default())
            .service(get_cards)
            .service(get_cards_json)
            .service(post_card_json)
            .service(put_card_json)
            .service(Files::new("/css", "static/css/"))
            .service(Files::new("/js", "static/js/"))
            .service(Files::new("/img", "static/img/"))
            .app_data(web::Json::<models::Card>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))
    })
    .bind(&bind)?
    .run()
    .await
}
