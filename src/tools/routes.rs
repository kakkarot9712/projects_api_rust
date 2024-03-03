use super::model::Tools;
use crate::utils::database::MongoClient;
use actix_web::{
    web::{self, Data},
    HttpResponse,
};

pub async fn get_tools(client: Data<MongoClient>) -> HttpResponse {
    let tools = client.get_all_tools().await;
    HttpResponse::Ok().json(tools)
}

impl Tools {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("").route(web::get().to(get_tools)));
        cfg.service(web::resource("/{tool_id}").route(web::get().to(
            |path: web::Path<String>| async {
                let tool_id = path.into_inner();
                HttpResponse::Ok().body(format!("{}", tool_id))
            },
        )));
    }
}
