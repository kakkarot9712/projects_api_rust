use super::model::Languages;
use crate::utils::database::MongoClient;
use actix_web::{
    web::{self, Data},
    HttpResponse,
};
// use mongodb::{Collection, Database};

pub async fn get_languages(client: Data<MongoClient>) -> HttpResponse {
    let langs = client.get_all_languages().await;
    HttpResponse::Ok().json(langs)
}

impl Languages {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("").route(web::get().to(get_languages)));
        cfg.service(web::resource("/{language_id}").route(web::get().to(
            |path: web::Path<String>| async {
                let language_id = path.into_inner();
                HttpResponse::Ok().body(format!("{}", language_id))
            },
        )));
    }
}
