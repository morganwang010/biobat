use super::schema::{ba, com, ele};
use diesel::sql_types::*;
use diesel::Queryable;
use diesel::QueryableByName;
use serde::Serialize;

#[derive(Debug, Clone, Queryable, Serialize, QueryableByName)]
pub struct Ba {
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
pub struct Com {
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

#[derive(Debug, Queryable, Serialize, QueryableByName)]
pub struct Ele {
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
