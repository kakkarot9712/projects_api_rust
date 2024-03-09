// use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use mongodb::bson::{oid::ObjectId, serde_helpers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tools {
    #[serde(serialize_with = "serde_helpers::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub name: String,
    pub svg_data: String,
}

// impl Responder for Tools {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         let body = serde_json::to_string(&self).unwrap();
//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(body)
//     }
// }
