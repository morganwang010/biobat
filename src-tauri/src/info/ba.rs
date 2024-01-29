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
    array: Vec<BaQueryItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaFilter {
    pub id: Option<i32>,
    pub limit: Option<i32>,
}

impl Ba {
    pub fn get_ba_with_ida(_id: String) -> Option<models::Ba> {
        let mut baid = _id.parse::<i32>().unwrap();
        let mut connection = establish_connection();
        let mut result = schema::ba::dsl::ba
            .filter(schema::ba::id.eq(&baid))
            .load::<models::Ba>(&mut connection)
            .unwrap_or(vec![]);

        return if result.len() == 1 {
            result.pop()
        } else {
            None
        };
    }
    // // pub fn get_ba_with_id(id: Integer) -> Option<models::Ba> {
    pub fn get_ba_with_id(_id: String) -> BaQueryResult {
        let mut baid = _id.parse::<i32>().unwrap();
        log::debug!("baid: {}", baid);
        println!("baid: {}", baid);
        let mut connection = establish_connection();
        let mut query = diesel::sql_query("").into_boxed();
        let mut limit = 12;
        let params = format!("?");
        query = query.sql(format!(
            "
                SELECT
                    *
                FROM
                    ba  where id = ({}) ",
            params
        ));
        query = query.bind::<Integer, _>(baid);
        // print!("query: {}", query);
        // println!("query: {}", query.);

        let result: Vec<BaQueryItem> = query
            .load::<BaQueryItem>(&mut connection)
            .expect("Expect loading bacteria info");

        return BaQueryResult { array: result };
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

        return BaQueryResult { array: result };
    }
}
