use super::schema::{ba, com, element};
use diesel::sql_types::*;
use serde::Serialize;
use diesel::Queryable;
use diesel::QueryableByName;

#[derive(Debug, Clone, Queryable, Serialize, QueryableByName)]
pub struct Ba {
  #[diesel(sql_type = i32)]
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



#[derive(Debug, Queryable, Serialize, QueryableByName)]
pub struct Com {
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


#[derive(Debug, Queryable, Serialize, QueryableByName)]
pub struct Element {
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

// #[derive(Debug, Clone, Serialize, Insertable)]
// #[diesel(table_name = feeds)]
// pub struct NewFeed {
//   pub uuid: String,
//   pub feed_type: String,
//   pub title: String,
//   pub link: String,
//   pub logo: String,
//   pub feed_url: String,
//   pub description: String,
//   pub pub_date: String,
//   pub updated: String,
//   pub sort: i32,
// // }
// #[derive(Debug, Insertable)]
// #[diesel(table_name = feed_metas)]
// pub struct NewFeedMeta {
//   pub uuid: String,
//   pub folder_uuid: String,
//   pub sort: i32,
// }

// #[derive(Debug, Queryable, Serialize, Associations, QueryableByName)]
// #[diesel(belongs_to(Feed, foreign_key = uuid))]
// pub struct Article {
//   #[diesel(sql_type = Integer)]
//   pub id: i32,

//   #[diesel(sql_type = Text)]
//   pub uuid: String,

//   #[diesel(sql_type = Text)]
//   pub title: String,

//   #[diesel(sql_type = Text)]
//   pub link: String,

//   #[diesel(sql_type = Text)]
//   pub feed_url: String,

//   #[diesel(sql_type = Text)]
//   pub feed_uuid: String,

//   #[diesel(sql_type = Text)]
//   pub description: String,

//   #[diesel(sql_type = Text)]
//   pub author: String,

//   #[diesel(sql_type = Text)]
//   pub pub_date: String,

//   #[diesel(sql_type = Text)]
//   pub content: String,

//   #[diesel(sql_type = Text)]
//   pub create_date: String,

//   #[diesel(sql_type = Text)]
//   pub update_date: String,

//   #[diesel(sql_type = Integer)]
//   pub read_status: i32,

//   #[diesel(sql_type = Text)]
//   pub media_object: Option<String>,

//   #[diesel(sql_type = Integer)]
//   pub starred: i32,
// }

// #[derive(Debug, Insertable, Clone)]
// #[diesel(table_name = articles)]
// pub struct NewArticle {
//   pub uuid: String,
//   pub feed_uuid: String,
//   pub title: String,
//   pub link: String,
//   pub feed_url: String,
//   pub description: String,
//   pub content: String,
//   pub author: String,
//   pub pub_date: String,
//   pub media_object: String,
// }

// #[derive(Debug, Queryable, QueryableByName, Clone, Serialize)]
// pub struct Folder {
//   #[diesel(sql_type = Integer)]
//   pub id: i32,
//   #[diesel(sql_type = Text)]
//   pub uuid: String,
//   #[diesel(sql_type = Text)]
//   pub name: String,
//   #[diesel(sql_type = Integer)]
//   pub sort: i32,
//   #[diesel(sql_type = Text)]
//   pub create_date: String,
//   #[diesel(sql_type = Text)]
//   pub update_date: String,
// }

// #[derive(Debug, Insertable, Clone)]
// #[diesel(table_name = folders)]
// pub struct NewFolder {
//   pub uuid: String,
//   pub name: String,
//   pub sort: i32,
// }
