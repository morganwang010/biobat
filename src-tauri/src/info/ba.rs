
use diesel::prelude::*;
use diesel::result;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

use crate::db::establish_connection;
use crate::models;
use crate::schema;

pub struct Ba {}
// #[derive(Debug, Serialize)]
// pub struct BaQueryResult {
//   list: Vec<BaQueryItem>,
// }   

#[derive(Debug, Queryable, Serialize, QueryableByName)]
pub struct BaQueryItem {
    #[diesel(sql_type = Integer)]
    pub id: i32,
  
    #[diesel(sql_type = Text)]
    pub number: String,
  
    #[diesel(sql_type = Text)]
    pub code: String,
  
    #[diesel(sql_type = Text)]
    pub nameen: String,
  
    #[diesel(sql_type = Text)]
    pub namecn: String,
  
    #[diesel(sql_type = Text)]
    pub source: String,
  
    #[diesel(sql_type = Text)]
    pub place: String,
  
    #[diesel(sql_type = Text)]
    pub org: String,
  
    #[diesel(sql_type = Text)]
    pub research: String,
  
    #[diesel(sql_type = Text)]
    pub sdate: String,
}

// use std::vec::Vec;
#[derive(Debug, Serialize)]
pub struct BaQueryResult {
    list: Vec<BaQueryItem>,
  }


#[derive(Debug, Serialize, Deserialize)]
pub struct BaFilter {
  pub id: Option<i32>,
  pub limit: Option<i32>,
}

impl Ba {
// pub fn get_ba_with_id(id: Integer) -> Option<models::Ba> {
pub fn get_ba_with_id(_id: String)  {
    // let mut connection = establish_connection();
    // let mut result = schema::ba::dsl::ba
    //   .filter(schema::ba::id.eq(&id))
    //   .load::<models::Ba>(&mut connection)
    //   .unwrap_or(vec![]);
    // let result = Vec::new();
    let a = "a".to_string();
    return if a.len() == 1 {
    //   result.push(1)
"it is ok".to_string();
    } else {
      "None".to_string();
    };
  }

pub fn get_ba(bf: BaFilter) -> BaQueryResult {
    let mut connection = establish_connection();
    let mut query = diesel::sql_query("").into_boxed();
    let mut limit = 12;
    query = query.sql(format!(
        "
            SELECT
             *
            FROM
              ba ",
      ));
      
    let result: Vec<BaQueryItem> = query
      .load::<BaQueryItem>(&mut connection)
      .expect("Expect loading bacteria info");

    return BaQueryResult { list: result };
  }
}