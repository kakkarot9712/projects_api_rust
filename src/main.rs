use actix_cors::Cors;
use actix_web::{http, web, App, HttpResponse, HttpServer};
use backend::languages::model::Languages;
use backend::tools::model::Tools;
use backend::utils::database::MongoClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoClient::init().await;
    let db = web::Data::new(db);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".vikalpg.in"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(db.clone())
            .service(web::scope("/languages").configure(Languages::config))
            .service(web::scope("/tools").configure(Tools::config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Its up!") }),
            )
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
