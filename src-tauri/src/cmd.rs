use serde::{Serialize};
use tauri::{command, Window};
// use tokio::sync::{mpsc, Mutex};

use crate::info;
use crate::models;

#[derive(Debug, Serialize)]
pub struct BaFetchResponse {
  ba: models::Ba,
  message: String,
}

#[command]
// pub async fn fetch_ba(id: String) -> () Option<models::Ba>, String{
pub async fn fetch_ba(id: String) -> () {
    println!("fetch_ba id: {}", id);
//   let res = Ba::parse_feed(&id).await;

//   match res {
//     Ok(res) => {
//       (Some(res), String::from(""))
//     }
//     Err(err) => (None, err),
//   }
}