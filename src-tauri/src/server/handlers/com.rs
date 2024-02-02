use actix_web::{get, post, web, Responder, Result};
use diesel::sql_types::Integer;
use serde::{Deserialize, Serialize};

use crate::info;

#[get("/api/com/{id}")]
pub async fn handle_get_com_with_id( id: web::Path<String>) -> Result<impl Responder> {
 
    let res = info::com::Com::get_com_with_id(id.to_string());

    Ok(web::Json(res))
}

#[get("/api/com")]
pub async fn handle_get_com(query: web::Query<info::com::ComFilter>) -> Result<impl Responder> {
    let filter = info::com::ComFilter {
        id: query.id.clone(),
        limit: query.limit.clone(),
    };

    let res = info::com::Com::get_com(filter);

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
    cfg.service(handle_get_com_with_id).service(handle_get_com);
}
