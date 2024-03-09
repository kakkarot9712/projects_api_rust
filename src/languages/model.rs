// use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use mongodb::bson::{oid::ObjectId, serde_helpers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Languages {
    pub name: String,
    #[serde(serialize_with = "serde_helpers::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub svg_data: String,
}

// impl Responder for Languages {
//     type Body = BoxBody;
//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         let body = serde_json::json!({"status": "success", "data": &self}).to_string();
//         println!("{}", body);
//         // let body = serde_json::to_string(&self).unwrap();
//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(body)
//     }
// }
