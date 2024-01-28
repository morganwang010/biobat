
use diesel::prelude::*;
use diesel::result;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

use crate::db::establish_connection;
use crate::models;
use crate::schema;
// use std::vec::Vec;
pub struct Ba {}


#[derive(Debug, Serialize, Deserialize)]
pub struct BaFilter {
  pub id: Option<i32>,
  pub limit: Option<i32>,
}
// pub fn get_ba_with_id(id: Integer) -> Option<models::Ba> {
pub fn get_ba_with_id(id: i32) -> Option<i32> {
    // let mut connection = establish_connection();
    // let mut result = schema::ba::dsl::ba
    //   .filter(schema::ba::id.eq(&id))
    //   .load::<models::Ba>(&mut connection)
    //   .unwrap_or(vec![]);
    // let result = Vec::new();
    let a = "a".to_string();
    return if a.len() == 1 {
    //   result.push(1)
    None
    } else {
      None
    };
  }

pub fn get_ba(bf: BaFilter) -> Option<i32> {
    // let mut connection = establish_connection();
    // let mut result = schema::ba::dsl::ba
    //   .filter(schema::ba::id.eq(&id))
    //   .load::<models::Ba>(&mut connection)
    //   .unwrap_or(vec![]);
    // let result = Vec::new();
    let a = "a".to_string();
    return if a.len() == 1 {
    //   result.push(1)
    None
    } else {
      None
    };
  }