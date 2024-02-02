use diesel::prelude::*;
use diesel::result;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

use crate::db::establish_connection;
use crate::models;
use crate::schema;

pub struct Com {}
// #[derive(Debug, Serialize)]
// pub struct ComQueryResult {
//   list: Vec<ComQueryItem>,
// }

#[derive(Debug, Queryable, Serialize, QueryableByName)]
pub struct ComQueryItem {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub number: String,
    #[diesel(sql_type = Text)]
    pub code: String,
    #[diesel(sql_type = Text)]
    pub source: String,
    #[diesel(sql_type = Text)]
    pub structure: String,
    #[diesel(sql_type = Text)]
    pub mol: String,
    #[diesel(sql_type = Text)]
    pub molfomula: String,
    #[diesel(sql_type = Text)]
    pub comno: String,
    #[diesel(sql_type = Text)]
    pub info: String,
    #[diesel(sql_type = Text)]
    pub new: String,
    #[diesel(sql_type = Text)]
    pub oneh: String,
    #[diesel(sql_type = Text)]
    pub cc: String,
    #[diesel(sql_type = Text)]
    pub hsqc: String,
    #[diesel(sql_type = Text)]
    pub hmbc: String,
    #[diesel(sql_type = Text)]
    pub cosy: String,
    #[diesel(sql_type = Text)]
    pub hrms: String,
    #[diesel(sql_type = Text)]
    pub ir: String,
    #[diesel(sql_type = Text)]
    pub uv: String,
    #[diesel(sql_type = Text)]
    pub xray: String,
    #[diesel(sql_type = Text)]
    pub note: String,
    #[diesel(sql_type = Text)]
    pub charger: String,
    #[diesel(sql_type = Text)]
    pub sdate: String,
}

// use std::vec::Vec;
#[derive(Debug, Serialize)]
pub struct ComQueryResult {
    array: Vec<ComQueryItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComFilter {
    pub id: Option<i32>,
    pub limit: Option<i32>,
}

impl Com {
    // pub fn get_com_with_id(_id: String) -> Option<models::Ba> {
    //     let mut baid = _id.parse::<i32>().unwrap();
    //     let mut connection = establish_connection();
    //     let mut result = schema::com::dsl::com
    //         .filter(schema::com::id.eq(&baid))
    //         .load::<models::Com>(&mut connection)
    //         .unwrap_or(vec![]);

    //     return if result.len() == 1 {
    //         result.pop()
    //     } else {
    //         None
    //     };
    // }
    // // pub fn get_ba_with_id(id: Integer) -> Option<models::Ba> {
    pub fn get_com_with_id(_id: String) -> ComQueryResult {
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
                    com  where id = ({}) ",
            params
        ));
        query = query.bind::<Integer, _>(baid);
        // print!("query: {}", query);
        // println!("query: {}", query.);

        let result: Vec<ComQueryItem> = query
            .load::<ComQueryItem>(&mut connection)
            .expect("Expect loading compound info");

        return ComQueryResult { array: result };
    }

    pub fn get_com(bf: ComFilter) -> ComQueryResult {
        let mut connection = establish_connection();
        let mut query = diesel::sql_query("").into_boxed();
        let mut limit = 12;
        query = query.sql(format!(
            "
            SELECT
             *
            FROM
              com ",
        ));

        let result: Vec<ComQueryItem> = query
            .load::<ComQueryItem>(&mut connection)
            .expect("Expect loading bacteria info");

        return ComQueryResult { array: result };
    }
}
