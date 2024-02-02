use actix_web::{get, post, web, Responder, Result};
use diesel::sql_types::Integer;
use serde::{Deserialize, Serialize};

use crate::info;

#[get("/api/ele/{id}")]
pub async fn handle_get_ele_with_id( id: web::Path<String>) -> Result<impl Responder> {
 
    let res = info::ele::Ele::get_ele_with_id(id.to_string());

    Ok(web::Json(res))
}

#[get("/api/ele")]
pub async fn handle_get_ele(query: web::Query<info::ele::EleFilter>) -> Result<impl Responder> {
    let filter = info::ele::EleFilter {
        id: query.id.clone(),
        limit: query.limit.clone(),
    };

    let res = info::ele::Ele::get_ele(filter);發地方

    Ok(web::Json(res))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadParam {
    read_status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarParam {
    starred: i32,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_get_ele_with_id).service(handle_get_ele);
}
