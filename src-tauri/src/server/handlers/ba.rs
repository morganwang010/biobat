use actix_web::{get, post, web, Responder, Result};
use diesel::sql_types::Integer;
use serde::{Deserialize, Serialize};

use crate::info;

#[get("/api/ba/{id}")]
pub async fn handle_get_ba_with_id( id: web::Path<String>) -> Result<impl Responder> {
 
    let res = info::ba::Ba::get_ba_with_id(id.to_string());

    Ok(web::Json(res))
}

#[get("/api/ba")]
pub async fn handle_get_ba(query: web::Query<info::ba::BaFilter>) -> Result<impl Responder> {
    let filter = info::ba::BaFilter {
        id: query.id.clone(),
        limit: query.limit.clone(),
    };

    let res = info::ba::Ba::get_ba(filter);

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
    cfg.service(handle_get_ba_with_id).service(handle_get_ba);
}
