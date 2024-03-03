use actix_web::{web, App, HttpResponse, HttpServer};
use backend::languages::model::Languages;
use backend::tools::model::Tools;
use backend::utils::database::MongoClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoClient::init().await;
    let db = web::Data::new(db);
    HttpServer::new(move || {
        App::new()
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
