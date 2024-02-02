use diesel::prelude::*;
use diesel::result;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

use crate::db::establish_connection;
use crate::models;
use crate::schema;

pub struct Ele {}
// #[derive(Debug, Serialize)]
// pub struct EleQueryResult {
//   list: Vec<EleQueryItem>,
// }

#[derive(Debug, Queryable, Serialize, QueryableByName)]
pub struct EleQueryItem {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub number: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub catlog: String,
    #[diesel(sql_type = Text)]
    pub class: String,
    #[diesel(sql_type = Text)]
    pub source: String,
    #[diesel(sql_type = Text)]
    pub describe: String,
    #[diesel(sql_type = Text)]
    pub detail: String,
    #[diesel(sql_type = Text)]
    pub size: String,
    #[diesel(sql_type = Text)]
    pub regno: String,
    #[diesel(sql_type = Text)]
    pub researcher: String,
    #[diesel(sql_type = Text)]
    pub seqinfo: String,
    #[diesel(sql_type = Text)]
    pub sdate: String,
}

// use std::vec::Vec;
#[derive(Debug, Serialize)]
pub struct EleQueryResult {
    array: Vec<EleQueryItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EleFilter {
    pub id: Option<i32>,
    pub limit: Option<i32>,
}

impl Ele {
    // pub fn get_ba_with_ida(_id: String) -> Option<models::Ele> {
    //     let mut baid = _id.parse::<i32>().unwrap();
    //     let mut connection = establish_connection();
    //     let mut result = schema::ba::dsl::ba
    //         .filter(schema::ba::id.eq(&baid))
    //         .load::<models::Ele>(&mut connection)
    //         .unwrap_or(vec![]);

    //     return if result.len() == 1 {
    //         result.pop()
    //     } else {
    //         None
    //     };
    // }
    // // pub fn get_ba_with_id(id: Integer) -> Option<models::Ele> {
    pub fn get_ele_with_id(_id: String) -> EleQueryResult {
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
                    ele  where id = ({}) ",
            params
        ));
        query = query.bind::<Integer, _>(baid);
        // print!("query: {}", query);
        // println!("query: {}", query.);

        let result: Vec<EleQueryItem> = query
            .load::<EleQueryItem>(&mut connection)
            .expect("Expect loading bacteria info");

        return EleQueryResult { array: result };
    }

    pub fn get_ele(bf: EleFilter) -> EleQueryResult {
        let mut connection = establish_connection();
        let mut query = diesel::sql_query("").into_boxed();
        let mut limit = 12;
        query = query.sql(format!(
            "
            SELECT
             *
            FROM
              ele ",
        ));

        let result: Vec<EleQueryItem> = query
            .load::<EleQueryItem>(&mut connection)
            .expect("Expect loading element info");

        return EleQueryResult { array: result };
    }
}
